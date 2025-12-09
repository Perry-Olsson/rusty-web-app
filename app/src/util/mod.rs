use actix_web::{
    dev::Payload,
    http::header,
    FromRequest,
    HttpRequest, 
    HttpResponse
};
use askama::Template;
use serde::Serialize;
use std::future::{ready, Ready};

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
        None => HttpResponse::NotFound().body("Not found")
    }
}
