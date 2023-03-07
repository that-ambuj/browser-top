use actix_cors::Cors;
use actix_files as fs;
use actix_web::{middleware::Logger, web, App, HttpServer};
use std::{thread, time::Duration};
use sysinfo::{CpuExt, System, SystemExt};
use tokio::sync::broadcast;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod routes;
mod ws_handler;

type Snapshot = Vec<f32>;

pub struct AppState {
    tx: broadcast::Sender<Snapshot>,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // env_logger::init_from_env(Env::default().default_filter_or("trace"));
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let (tx, _) = broadcast::channel::<Snapshot>(1);
    let app_state = web::Data::new(AppState { tx: tx.clone() });

    let _ = thread::spawn(move || {
        let mut sys = System::new();

        loop {
            sys.refresh_cpu();
            let v: Vec<_> = sys.cpus().iter().map(|cpu| cpu.cpu_usage()).collect();
            let _ = tx.send(v);

            std::thread::sleep(Duration::from_millis(200));
        }
    });

    let addr = ("0.0.0.0", 8000);

    println!(
        "Starting server on http://{}:{} and http://localhost:{}",
        &addr.0, &addr.1, &addr.1
    );

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .configure(routes::configure)
            .service(
                fs::Files::new("/", "./frontend/dist/")
                    .index_file("index.html")
                    .show_files_listing(),
            )
            .wrap(Cors::default().allow_any_origin().allow_any_method())
            .app_data(app_state.clone())
    })
    .client_request_timeout(Duration::from_secs(2))
    .shutdown_timeout(3)
    .workers(2)
    .bind(&addr)?
    .run()
    .await?;

    Ok(())
}
