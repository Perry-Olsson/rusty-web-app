use actix_web::{get, App, HttpResponse, HttpServer, Responder};

const PORT: u16 = 8080;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("hello world\n")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(|| {
        App::new()
            .service(hello)
    })
    .bind(("0.0.0.0", PORT))?
    .run();
    println!("Server listening on port {}", PORT);
    server.await
}
