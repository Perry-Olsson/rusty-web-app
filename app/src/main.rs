use actix_web;
use app;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    app::run().await
}
