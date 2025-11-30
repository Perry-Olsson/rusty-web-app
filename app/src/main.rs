use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

const PORT: u16 = 8080;

#[derive(Serialize, Deserialize)]
struct Post {
    title: String,
    content: String,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("hello world\n")
}

#[get("/post")]
async fn get_post() -> impl Responder {
    let post = _get_post();
    web::Json(post)
}

fn _get_post() -> Post {
    Post {
        title: "My First Post".to_string(),
        content: "This is some dummy content for the post.".to_string(),
    }
}

#[post("/post")]
async fn create_post() -> impl Responder {
    HttpResponse::Ok().body("POST /post\n")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(get_post)
            .service(create_post)
    })
    .bind(("0.0.0.0", PORT))?
    .run();
    println!("Server listening on port {}", PORT);
    server.await
}
