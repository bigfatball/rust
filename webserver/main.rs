use actix_files as fs;
use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(fs::Files::new("/", "../app/dist").index_file("index.html")))
        .bind(("192.168.248.128", 8080))?
        .run()
        .await
}