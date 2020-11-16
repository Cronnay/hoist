use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use std::sync::{Mutex};
use std::collections::HashMap;
use std::net::TcpListener;
use crate::models::pod::{Pod, MappedPort};
use crate::docker;
use crate::docker::container as DockerContainer;
use crate::docker::image as DockerImage;


#[derive(Debug, Deserialize, Serialize)]
pub struct CreatePodRequest {
    name: String,
    slug: String,
    port: u16,
    docker_image: String,
    docker_tag: String
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PodInfoRequest {
    slug: String
}


pub async fn create_pod(all_pods: web::Data<Mutex<HashMap<String, Pod>>>, req: web::Json<CreatePodRequest>) -> HttpResponse {
    let docker_instance = docker::get_docker_instance();
    
    if let Err(_) = DockerImage::get_image_by_name(format!("{}:{}", &req.docker_image, &req.docker_tag), &docker_instance).await {
        println!("Image does not exists. Attempt to pull");
        let image = DockerImage::pull_image(req.docker_image.clone(), req.docker_tag.clone(), &docker_instance).await;
        match image {
            Ok(_) => { println!("Image has been pulled")},
            Err(e) => println!("Error while pulling: {}", e)
        };
    }

    let port_map = MappedPort::new(req.port, get_available_port());
    let created_pod = Pod::new(req.name.clone(), req.slug.clone(), req.docker_image.clone(), req.docker_tag.clone(), port_map);
    
    DockerContainer::create_and_start_docker_container(&docker_instance, &created_pod).await;
    
    let mut pods = all_pods.lock().unwrap();
    pods.insert(created_pod.slug.to_string(), created_pod);

    HttpResponse::Ok().body("Request Complete")
}

/// Will lists all pods running
pub fn list_all_pods(all_pods: web::Data<Mutex<HashMap<String, Pod>>>) -> HttpResponse {
    let pods = all_pods.lock().unwrap();
    HttpResponse::Ok().json(pods.clone())
}

/// Get information about pod from slug
pub fn get_pod(all_pods: web::Data<Mutex<HashMap<String, Pod>>>, web::Path(slug): web::Path<String>) -> HttpResponse {
    let pods = all_pods.lock().unwrap();
    if pods.contains_key(&slug) {
        HttpResponse::Ok().json(pods.get(&slug))
    } else {
        HttpResponse::NotFound().body(format!("Did not found a pod with slug: {}", slug))
    }
}
/// Updates the pod. Client provides the slug and also a body of the update
pub fn update_pod(all_pods: web::Data<Mutex<HashMap<String, Pod>>>, req: web::Json<PodInfoRequest>, web::Path(slug): web::Path<String>) -> HttpResponse {
    let pods = all_pods.lock().unwrap();
    if pods.contains_key(&slug) {
        HttpResponse::Ok().body("helo")
    } else {
        HttpResponse::NotFound().body(format!("Did not found a pod with slug: {}", req.slug))
    }

}

fn get_available_port() -> Option<u16> {
    (24000..65535).find(|port| port_is_available(*port))
}

fn port_is_available(port: u16) -> bool {
    match TcpListener::bind(("127.0.0.1", port)) {
        Ok(_) => true,
        Err(_) => false,
    }
}