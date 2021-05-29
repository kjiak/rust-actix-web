mod request_handlers;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(request_handlers::hello)
            .service(request_handlers::echo)
            .route("/hey", web::get().to(request_handlers::manual_hello))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}