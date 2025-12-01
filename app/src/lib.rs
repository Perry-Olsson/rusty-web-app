mod controllers;
mod models;
mod service;

use actix_web::{get, App, HttpResponse, HttpServer, Responder};

use controllers::post_controller;

const PORT: u16 = 8080;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("hello world\n")
}

pub async fn run() -> std::io::Result<()> {
    let server = HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(post_controller::create())
    })
    .bind(("0.0.0.0", PORT))?
    .run();
    println!("Server listening on port {}", PORT);
    server.await
}
