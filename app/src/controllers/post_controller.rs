use actix_web::{
    get, post, web::{self}, HttpResponse, Responder, Scope
};
use serde::Deserialize;
use crate::{
    models::id::Id, service::post_service::{GetPostRequest, NewPost, PostService}, util::{respond_optional, ContentType}
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

#[get("/{id}")]
pub async fn get_post(
    services: web::Data<PostData>,
    path: web::Path<Id>,
    query: web::Query<GetPostQuery>,
    fmt: ContentType,
) -> impl Responder {
    let maybe_post = services.service.get_post(
        GetPostRequest {
            id: path.into_inner(),
            upper: query.upper
        }
        );

    respond_optional(maybe_post, fmt)
}

#[derive(Deserialize)]
struct GetPostQuery {
    upper: Option<bool>,
}

#[post("")]
pub async fn create_post(services: web::Data<PostData>, post: web::Json<NewPost>) -> impl Responder {
    match services.service.create_post(post.into_inner()) {
        Ok(post) => HttpResponse::Ok().json(post),
        Err(err) => HttpResponse::BadRequest().body(err),
    }
}
