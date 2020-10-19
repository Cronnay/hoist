mod docker;
use actix_web::{get, App, HttpServer, Responder};
use docker::get_docker_instance;
use docker::image::get_all_images;

#[get("/images")]
async fn index() -> impl Responder {
    let docker = get_docker_instance();
    let images = get_all_images(&docker).await.unwrap();
    format!("Images: {:?}", images)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
