use actix_web::{
    dev::BodyEncoding,
    web, get, middleware, App, HttpResponse, HttpServer,
    http::ContentEncoding,
};

async fn response() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("plain/text")
        .header("X-Hdr", "sample")
        .body("data")
}

#[get("/index_br")]
async fn index_br() -> HttpResponse {
    HttpResponse::Ok()
        .encoding(ContentEncoding::Br)
        .body("data")
}

static GZIP_HELLO_WORLD: &[u8] = &[
    0x1f, 0x8b, 0x08, 0x00, 0xa2, 0x30, 0x10, 0x5c, 0x00, 0x03, 0xcb, 0x48, 0xcd, 0xc9, 0xc9, 0x57,
    0x28, 0xcf, 0x2f, 0xca, 0x49, 0xe1, 0x02, 0x00, 0x2d, 0x3b, 0x08, 0xaf, 0x0c, 0x00, 0x00, 0x00,
];


#[get("/gzip")]
async fn gzip() -> HttpResponse {
    HttpResponse::Ok()
        .encoding(ContentEncoding::Identity)
        .header("content-encoding", "gzip")
        .body(GZIP_HELLO_WORLD)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(||
        App::new()
            .route("/response", web::get().to(response))
            .wrap(middleware::Compress::default())
            .service(index_br)
            .service(gzip)
    )
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
