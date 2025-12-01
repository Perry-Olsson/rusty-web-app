use actix_web::{
    dev::Payload,
    http::header,
    FromRequest,
    HttpRequest,
};
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
