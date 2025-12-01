use actix_web::{get, post, web, HttpResponse, Responder};

use crate::service::post_service::{GetPostQuery, NewPost, PostService};

#[get("/post")]
pub async fn get_post(query: web::Query<GetPostQuery>) -> impl Responder {
    let post = PostService::new().get_post(&query);
    web::Json(post)
}

#[post("/post")]
pub async fn create_post(post: web::Json<NewPost>) -> impl Responder {
    match PostService::new().create_post(post.into_inner()) {
        Ok(post) => HttpResponse::Ok().json(post),
        Err(err) => HttpResponse::BadRequest().body(err),
    }
}
