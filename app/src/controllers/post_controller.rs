use actix_web::{get, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Post {
    pub title: String,
    pub content: String,
}

#[get("/post")]
pub async fn get_post() -> impl Responder {
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
pub async fn create_post() -> impl Responder {
    HttpResponse::Ok().body("POST /post\n")
}
