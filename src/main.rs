mod controllers;
mod docker;
use actix_web::{App, HttpServer};
use controllers::containers::{get_container_by_name, get_containers};
use controllers::images::{get_image_from_name, get_images};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_images)
            .service(get_image_from_name)
            .service(get_container_by_name)
            .service(get_containers)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
