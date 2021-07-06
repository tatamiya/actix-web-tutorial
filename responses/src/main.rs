use actix_web::{web, App, HttpResponse, HttpServer};

async fn response() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("plain/text")
        .header("X-Hdr", "sample")
        .body("data")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(||
        App::new()
            .route("/response", web::get().to(response))
    )
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
