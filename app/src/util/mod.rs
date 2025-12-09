use actix_web::{
    dev::{Payload},
    http::{header},
    FromRequest,
    HttpRequest, 
    HttpResponse 
};
use askama::Template;
use serde::Serialize;
use std::{future::{ready, Ready}};

use crate::models::error::ErrorResponse;

pub enum ResponseFmt {
    HTML,
    JSON
}

impl FromRequest for ResponseFmt {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let accept = req.headers().get(header::ACCEPT);
        let fmt = match accept.and_then(|h| h.to_str().ok()) {
            Some(s) if s.contains("application/json") => ResponseFmt::JSON,
            _ => ResponseFmt::HTML,
        };
        ready(Ok(fmt))
    }
}

pub fn respond<T>(res: T, fmt: ResponseFmt) -> HttpResponse 
where T: Serialize + Template {
    match fmt {
        ResponseFmt::HTML => {
            match res.render() {
                Ok(html) => HttpResponse::Ok()
                    .content_type("text/html; charset=utf-8")
                    .body(html),
                Err(err) => HttpResponse::InternalServerError()
                    .body(format!("Template error: {}", err)),
            }
        },
        ResponseFmt::JSON => {
            HttpResponse::Ok().json(res)
        },
    }
}

pub fn respond_optional<T>(maybe_res: Option<T>, fmt: ResponseFmt) -> HttpResponse
where T: Serialize + Template {
    match maybe_res {
        Some(res) => respond(res, fmt),
        None => handle_not_found(fmt)
    }
}

// TODO find a solid solution for serving these static files as is.
#[derive(Template)]
#[template(path = "errors/404.html")]
pub struct NotFound;

fn handle_not_found(fmt: ResponseFmt) -> HttpResponse {
    match fmt {
        ResponseFmt::HTML => {
            HttpResponse::NotFound().body(NotFound{}.render().unwrap())
        },
        ResponseFmt::JSON => {
            HttpResponse::NotFound().json(ErrorResponse { message: "Resource Not Found".to_string() })
        },
    }
}
