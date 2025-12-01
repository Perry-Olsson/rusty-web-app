use actix_web::{
    get,
    post,
    web::{self},
    HttpResponse,
    Responder,
    Scope
};
use askama::Template;
use crate::{
    service::post_service::{GetPostQuery, NewPost, PostService},
    util::ResponseFmt
};

pub struct PostData {
    service: PostService
}

pub fn create() -> Scope {
    let post_scope = web::scope("/posts");
    post_scope.app_data(web::Data::new(PostData {
                service: PostService::new()
            }))
            .service(get_post)
            .service(create_post)
}

#[get("")]
pub async fn get_post(
    services: web::Data<PostData>,
    query: web::Query<GetPostQuery>,
    fmt: ResponseFmt,
) -> impl Responder {
    let post = services.service.get_post(&query);

    match fmt {
        ResponseFmt::HTML => {
            match post.render() {
                Ok(html) => HttpResponse::Ok()
                    .content_type("text/html; charset=utf-8")
                    .body(html),
                Err(err) => HttpResponse::InternalServerError()
                    .body(format!("Template error: {}", err)),
            }
        },
        ResponseFmt::JSON => {
            HttpResponse::Ok().json(post)
        },
    }
}

#[post("")]
pub async fn create_post(services: web::Data<PostData>, post: web::Json<NewPost>) -> impl Responder {
    match services.service.create_post(post.into_inner()) {
        Ok(post) => HttpResponse::Ok().json(post),
        Err(err) => HttpResponse::BadRequest().body(err),
    }
}
