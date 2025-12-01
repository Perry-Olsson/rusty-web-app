use actix_web::{
    dev::Payload,
    http::header,
    FromRequest,
    HttpRequest, HttpResponse, Responder,
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

pub fn send_response<T>(res: T, fmt: ResponseFmt) -> impl Responder 
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
