use crate::AppState;
use std::time::{Duration, Instant};

use actix_web::web;
use actix_ws::Message;
use futures_util::{
    future::{self, Either},
    StreamExt as _,
};
use tokio::{pin, time::interval};
/// Should be half (or less) of the acceptable client timeout.
const HEARTBEAT_INTERVAL: Duration = Duration::from_millis(200);

/// How long before lack of client response causes a timeout.
const CLIENT_TIMEOUT: Duration = Duration::from_secs(5);

pub async fn cpu_stats_ws(
    mut session: actix_ws::Session,
    mut msg_stream: actix_ws::MessageStream,
    state: web::Data<AppState>,
) {
    tracing::info!("connected websocket");

    let mut rx = state.tx.subscribe();

    let mut last_heartbeat = Instant::now();
    let mut interval = interval(HEARTBEAT_INTERVAL);

    let reason = loop {
        // create "next client timeout check" future
        // pin! is required for select()
        let tick = interval.tick();
        pin!(tick);

        let cpu_info = rx.recv();
        pin!(cpu_info);

        let messages = future::select(msg_stream.next(), tick);
        pin!(messages);

        // waits for either `msg_stream` to receive a message from the client or the heartbeat
        // interva timer to tick, yielding the value of whichever one is ready first
        match future::select(cpu_info, messages).await {
            // received message from WebSocket client
            Either::Left((Ok(info), _)) => {
                last_heartbeat = Instant::now();

                let payload = serde_json::to_string(&info).unwrap();
                let _ = session.text(payload).await;
            }

            // handle broadcast recieve error
            Either::Left((Err(err), _)) => {
                log::error!("{}", err);
                break None;
            }

            // handle Pings, Pongs and Heartbeats
            Either::Right((right, _)) => match right {
                // Play ping-pong here and handle closed connections
                Either::Left((Some(Ok(msg)), _)) => {
                    match msg {
                        Message::Close(reason) => {
                            break reason;
                        }

                        Message::Ping(bytes) => {
                            last_heartbeat = Instant::now();
                            let _ = session.pong(&bytes).await;
                        }

                        Message::Pong(_) => {
                            last_heartbeat = Instant::now();
                        }

                        Message::Continuation(_) => {
                            log::warn!("no support for continuation frames");
                        }

                        // ignore
                        _ => {}
                    };
                }

                // client WebSocket stream error
                Either::Left((Some(Err(err)), _)) => {
                    log::error!("{}", err);
                    break None;
                }

                // client WebSocket stream ended
                Either::Left((None, _)) => break None,

                // Handle heartbeats
                // heartbeat interval ticked
                Either::Right((_inst, _)) => {
                    // if no heartbeat ping/pong received recently, close the connection
                    if Instant::now().duration_since(last_heartbeat) > CLIENT_TIMEOUT {
                        tracing::info!(
                        "client has not sent heartbeat in over {CLIENT_TIMEOUT:?}; disconnecting"
                    );

                        break None;
                    }

                    // send heartbeat ping
                    let _ = session.ping(b"").await;
                }
            },
        }
    };

    let _ = session.close(reason).await;

    tracing::info!("disconnected websocket");
}
