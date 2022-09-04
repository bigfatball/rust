//use actix_files::NamedFile;

use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;


mod server;
use self::server::MyWebSocket;

use actix_http;
async fn websocket(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    ws::WsResponseBuilder::new(MyWebSocket::new(),&req,stream)
    .codec(actix_http::ws::Codec::new())
    .frame_size(10737418240)
    //.frame_size(65536)
    .protocols(&["A","B"])
    .start()
    //start(MyWebSocket::new(), &req, stream)
}

// async fn index() -> impl Responder {
//     NamedFile::open_async("../app/dist/index.html").await.unwrap()
// }


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        
        // .service(
        //     web::resource("/").to(index)
        // )
        
     
        
        .service(
            web::resource("/").route(web::get().to(websocket))
        )
        

    })
    .workers(4)
    .bind(("192.168.65.131", 8080))?
    .run()
    .await



}
