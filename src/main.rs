use actix_cors::Cors;
use actix_files as fs;
use actix_web::{get, web, App, HttpServer, Responder};
use std::sync::Mutex;
use sysinfo::{CpuExt, System, SystemExt};

struct AppState {
    sys: Mutex<System>,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        sys: Mutex::new(System::new()),
    });

    HttpServer::new(move || {
        App::new()
            .service(system_info)
            .service(
                fs::Files::new("/", "./frontend/dist/")
                    .index_file("index.html")
                    .show_files_listing(),
            )
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:5173")
                    .allow_any_method(),
            )
            .app_data(app_state.clone())
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}

#[get("/api/info")]
async fn system_info(state: web::Data<AppState>) -> impl Responder {
    // FIXME: It's blocking in async
    let mut sys = state.sys.lock().unwrap();
    sys.refresh_cpu();

    let vec: Vec<_> = sys.cpus().iter().map(|cpu| cpu.cpu_usage()).collect();

    web::Json(vec)
}
