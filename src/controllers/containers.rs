use crate::docker::container::{get_docker_container, get_docker_containers};
use crate::docker::get_docker_instance;
use actix_web::{get, web, HttpResponse};

#[get("/container")]
pub async fn get_containers() -> HttpResponse {
    let docker = get_docker_instance();
    let containers = get_docker_containers(&docker).await.unwrap(); // If panic - bigger problem then
    HttpResponse::Ok().json(containers)
}

#[get("/container/{name}")]
pub async fn get_container_by_name(web::Path(name): web::Path<String>) -> HttpResponse {
    let docker = get_docker_instance();
    let container = get_docker_container(&name, &docker).await;
    match container {
        Ok(c) => HttpResponse::Ok().json(c),
        Err(_) => {
            HttpResponse::BadRequest().body(format!("Container doesn't exist with name: {}", name))
        }
    }
}
