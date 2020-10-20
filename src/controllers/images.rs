use crate::docker::get_docker_instance;
use crate::docker::image::{get_all_images, get_image_by_name};
use actix_web::{get, web, HttpResponse};

#[get("/image")]
pub async fn get_images() -> HttpResponse {
    let docker = get_docker_instance();
    let images = get_all_images(&docker).await.unwrap();
    HttpResponse::Ok().json(images)
}

#[get("/image/{name}")]
pub async fn get_image_from_name(web::Path(name): web::Path<String>) -> HttpResponse {
    let docker = get_docker_instance();
    let image = get_image_by_name(&name, &docker).await;
    match image {
        Ok(img) => HttpResponse::Ok().json(img),
        Err(_) => HttpResponse::BadRequest().body(format!("Cannot find image with name {}", name)),
    }
}
