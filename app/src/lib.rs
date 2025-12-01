mod controllers;
mod models;
mod service;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

use controllers::post_controller::{create_post, get_post};

use crate::service::post_service::PostService;

const PORT: u16 = 8080;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("hello world\n")
}

pub struct Services {
    post_service: PostService
}

pub async fn run() -> std::io::Result<()> {
    let server = HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(Services {
                post_service: PostService::new()
            }))
            .service(hello)
            .service(get_post)
            .service(create_post)
    })
    .bind(("0.0.0.0", PORT))?
    .run();
    println!("Server listening on port {}", PORT);
    server.await
}
