use actix_web::{get, web, App, HttpServer, Responder};
use std::{fmt::Write, sync::Mutex};
use sysinfo::{CpuExt, System, SystemExt};

struct AppState {
    sys: Mutex<System>,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        sys: Mutex::new(System::new()),
    });

    HttpServer::new(move || App::new().service(index).app_data(app_state.clone()))
        .bind(("0.0.0.0", 8000))?
        .run()
        .await
}

#[get("/")]
async fn index(state: web::Data<AppState>) -> impl Responder {
    let mut s = String::new();

    let mut sys = state.sys.lock().unwrap();
    sys.refresh_cpu();

    for (i, cpu) in sys.cpus().iter().enumerate() {
        let i = i + 1;

        let usage = cpu.cpu_usage();
        writeln!(&mut s, "CPU {i}: {usage}").unwrap();
    }

    s
}
