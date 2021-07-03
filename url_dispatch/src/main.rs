use actix_web::{guard, web, App, HttpResponse, HttpServer};

async fn hello() -> HttpResponse {
    HttpResponse::Ok().body("Hello")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(hello))
            .route("/hello", web::post().to(hello))
            .service(
                web::resource("/hello2/{name}")
                    .name("hello_detail")
                    .guard(guard::Header("content-type", "application/json"))
                    .route(web::get().to(|| HttpResponse::Ok()))
                    .route(web::put().to(|| HttpResponse::Ok()))
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
