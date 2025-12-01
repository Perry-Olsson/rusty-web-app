use actix_web::{http::header::Accept, web::Header};

pub fn get_fmt(accept_header: Header<Accept>) -> ResponseFmt {
    for item in accept_header.iter() {
        let str = item.to_string();
        if str == "text/html" {
            return ResponseFmt::HTML
        } else if str == "application/json" {
            return ResponseFmt::JSON
        }
    }
    ResponseFmt::HTML
}

pub enum ResponseFmt {
    HTML,
    JSON
}
