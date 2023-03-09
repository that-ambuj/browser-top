use actix_cors::Cors;
use actix_files as fs;
use actix_web::{get, http::header::ContentType, middleware::Logger, web, App, HttpServer};
use actix_web::{HttpRequest, HttpResponse, Responder};
use fs::file_extension_to_mime;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::PathBuf;
use std::{thread, time::Duration};
use sysinfo::{CpuExt, System, SystemExt};
use tokio::sync::broadcast;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use walkdir::WalkDir;

mod routes;
mod ws_handler;

type Snapshot = Vec<f32>;

const FRONTEND_DIR: &'static str = "./frontend/dist";

// Include built and bundled frontend files inside the binary
lazy_static! {
    static ref FILEMAP: HashMap<String, Vec<u8>> = {
        let mut m = HashMap::new();

        for file in WalkDir::new(FRONTEND_DIR)
            .into_iter()
            .filter_map(|file| file.ok())
            .filter(|file| file.metadata().unwrap().is_file())
        {
            // includes frontend directory prefix `./frontend/dist`
            let path = file.path().to_str().unwrap();
            // does not include frontend directory prefix `./frontend/dist`
            let stripped_path = file
                .path()
                .strip_prefix(FRONTEND_DIR)
                .unwrap()
                .to_str()
                .unwrap();

            file.file_type();

            let f = File::open(path).unwrap();
            let mut reader = BufReader::new(f);
            let mut buffer = Vec::new();

            reader.read_to_end(&mut buffer).unwrap();

            m.insert(
                stripped_path.to_string(),
                buffer
            );
        }
        m
    };
}

pub struct AppState {
    tx: broadcast::Sender<Snapshot>,
}

#[get("/")]
async fn default_html() -> impl Responder {
    if let Some(file_data) = FILEMAP.get("index.html") {
        return HttpResponse::Ok()
            .content_type(ContentType::html())
            .body(file_data.clone());
    }

    HttpResponse::NotFound().body("File not found")
}

#[get("/{filename:.+(html|ico|css|js|png|svg|jpg)$}")]
async fn serve_files(req: HttpRequest) -> Result<impl Responder, actix_web::Error> {
    let file_name: PathBuf = req.match_info().query("filename").parse()?;
    let ext = file_name.extension().unwrap().to_str().unwrap();

    tracing::debug!("requested: {file_name:?}");

    let content_type = ContentType(file_extension_to_mime(ext));

    if let Some(file_data) = FILEMAP.get(file_name.to_str().unwrap()) {
        return Ok(HttpResponse::Ok()
            .content_type(content_type)
            .body(file_data.clone()));
    }

    Ok(HttpResponse::NotFound().body("File not found"))
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // env_logger::init_from_env(Env::default().default_filter_or("trace"));
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "debug".into()),
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
            .service(serve_files)
            .service(default_html)
            // .service(fs::Files::new("/", "./frontend/dist/").index_file("index.html"))
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
