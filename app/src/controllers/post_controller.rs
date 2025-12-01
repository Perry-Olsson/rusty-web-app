use actix_web::{get, http::header::{Accept, ContentType, QualityItem}, mime::Mime, post, web::{self, Header}, HttpRequest, HttpResponse, Responder, Scope};

use crate::{models::post::Post, service::post_service::{GetPostQuery, NewPost, PostService}, util::{get_fmt, ResponseFmt}};


pub struct PostData {
    service: PostService
}

pub fn create() -> Scope {
    let post_scope = web::scope("/post");
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
    accept: web::Header<Accept>
) -> impl Responder {
    let post = services.service.get_post(&query);

    match get_fmt(accept) {
        ResponseFmt::HTML => {
        HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(to_html(post))
        },
        ResponseFmt::JSON => {
            HttpResponse::Ok().json(post)
        },
    }
}

fn to_html(post: Post) -> String {
    format!(
            r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>{}</title>
</head>
<body>
    <h1>{}</h1>
    <p>{}</p>
    <hr>
    <small>Post ID: {}</small>
</body>
</html>"#,
            post.title, post.title, post.content, post.id
        )
}

#[post("")]
pub async fn create_post(services: web::Data<PostData>, post: web::Json<NewPost>) -> impl Responder {
    match services.service.create_post(post.into_inner()) {
        Ok(post) => HttpResponse::Ok().json(post),
        Err(err) => HttpResponse::BadRequest().body(err),
    }
}
