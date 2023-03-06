use actix_web::{get, App, HttpServer, Responder};

struct AppState {}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind(("0.0.0.0", 8000))?
        .run()
        .await
}

#[get("/")]
async fn index() -> impl Responder {
    "Hello"
}
