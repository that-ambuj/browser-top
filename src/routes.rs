use crate::ws_handler;
use crate::AppState;
use actix_web::{get, rt, web, HttpRequest, HttpResponse, Responder};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(system_info).service(echo_heartbeat_ws);
}

#[get("/api/info")]
async fn system_info(state: web::Data<AppState>) -> impl Responder {
    let mut rx = state.tx.subscribe();

    while let Ok(msg) = rx.recv().await {
        return web::Json(msg);
    }

    web::Json(Vec::with_capacity(8))
}

#[get("/ws/cpu")]
async fn echo_heartbeat_ws(
    req: HttpRequest,
    stream: web::Payload,
    state: web::Data<AppState>,
) -> Result<HttpResponse, actix_web::Error> {
    let (res, session, msg_stream) = actix_ws::handle(&req, stream)?;
    rt::spawn(ws_handler::cpu_stats_ws(session, msg_stream, state));

    Ok(res)
}
