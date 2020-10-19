use crate::docker::{get_docker_instance, image::get_all_images};
use actix_web::{get, Responder};

#[get("/images")]
pub async fn index() -> impl Responder {
    let docker = get_docker_instance();
    let images = get_all_images(&docker).await.unwrap();
    format!("Images: {:?}", images)
}
