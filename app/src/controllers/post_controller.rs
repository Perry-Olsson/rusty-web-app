use actix_web::{get, post, web, HttpResponse, Responder};

use crate::{service::post_service::{GetPostQuery, NewPost}, Services};

#[get("/post")]
pub async fn get_post(services: web::Data<Services>, query: web::Query<GetPostQuery>) -> impl Responder {
    let post = services.post_service.get_post(&query);
    web::Json(post)
}

#[post("/post")]
pub async fn create_post(services: web::Data<Services>, post: web::Json<NewPost>) -> impl Responder {
    match services.post_service.create_post(post.into_inner()) {
        Ok(post) => HttpResponse::Ok().json(post),
        Err(err) => HttpResponse::BadRequest().body(err),
    }
}
