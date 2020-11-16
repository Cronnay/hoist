use actix_web::{middleware, web, App, HttpServer};
use controllers::pods_controller::{create_pod, list_all_pods, get_pod, update_pod};
use models::pod::Pod;
use std::sync::Mutex;
use std::collections::HashMap;

mod controllers;
mod docker;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    let running_pods: web::Data<Mutex<HashMap<String, Pod>>> = web::Data::new(Mutex::new(HashMap::new()));
    HttpServer::new(move || {
        App::new()
            .app_data(running_pods.clone())
            .service(
                web::resource("/pods")
                    .route(web::post().to(create_pod))
                    .route(web::get().to(list_all_pods))
                )
            .service(
                web::resource("/pods/{slug}")
                .route(web::get().to(get_pod))
                .route(web::put().to(update_pod))
            )
            .wrap(middleware::Logger::default())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
