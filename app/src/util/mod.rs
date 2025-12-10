use actix_web::{
    http::{header::{self, Accept}, StatusCode}, mime, web::{self, Header}, HttpResponse 
};
use askama::Template;
use serde::Serialize;

use crate::models::error::ErrorResponse;

pub enum ContentType {
    HTML,
    JSON
}

pub fn respond_optional<T>(maybe_res: Option<T>, accept: Header<Accept>) -> HttpResponse
where T: Serialize + Template {
    handle_unsupported_content_type(accept, |content_type| {
        match maybe_res {
            Some(res) => _respond(res, content_type),
            None => handle_not_found(content_type)
        }
    })
}

pub fn respond<T>(res: T, accept: Header<Accept>) -> HttpResponse 
where T: Serialize + Template {
    handle_unsupported_content_type(accept, |content_type| {
        _respond(res, content_type)
    })
}

fn handle_unsupported_content_type<F>(accept: Header<Accept>, respond: F) -> HttpResponse
where 
    F: FnOnce(ContentType) -> HttpResponse,
{
    let maybe_content_type = get_content_type(accept);
    match maybe_content_type {
        Some(content_type) => {
            respond(content_type)
        },
        None => HttpResponse::new(StatusCode::NOT_ACCEPTABLE),
    }
}

fn _respond<T>(res: T, content_type: ContentType) -> HttpResponse 
where T: Serialize + Template {
    match content_type {
        ContentType::HTML => {
            match res.render() {
                Ok(html) => HttpResponse::Ok()
                    .content_type("text/html; charset=utf-8")
                    .body(html),
                Err(err) => HttpResponse::InternalServerError()
                    .body(format!("Template error: {}", err)),
            }
        },
        ContentType::JSON => {
            HttpResponse::Ok().json(res)
        },
    }
}

// TODO refactor this
fn get_content_type(accept: web::Header<header::Accept>) -> Option<ContentType> {
    for item in accept.iter() {
        if item.item == mime::TEXT_HTML || item.item == mime::STAR_STAR || item.item == mime::TEXT_STAR {
            return Some(ContentType::HTML);
        } else if item.item == mime::APPLICATION_JSON {
            return Some(ContentType::JSON);
        }
    }
    None
}

// TODO find a solid solution for serving these static files as is.
#[derive(Template)]
#[template(path = "errors/404.html")]
pub struct NotFound;

#[derive(Template)]
#[template(path = "errors/406-unsupported-browser.html")]
pub struct NotAcceptable;

fn handle_not_found(content_type: ContentType) -> HttpResponse {
    match content_type {
        ContentType::HTML => {
            HttpResponse::NotFound().body(NotFound{}.render().unwrap())
        },
        ContentType::JSON => {
            HttpResponse::NotFound().json(ErrorResponse { message: "Resource Not Found".to_string() })
        },
    }
}
