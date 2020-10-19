mod controllers;
mod docker;
use actix_web::{App, HttpServer};
use controllers::first_page::index;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
