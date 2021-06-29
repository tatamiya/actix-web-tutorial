use actix_web::{get, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};
use futures::{future::{ok, ready, Ready}, stream::once};
use serde::Serialize;
use serde_json;

#[derive(Serialize)]
struct MyObj {
    name: &'static str,
}

impl Responder for MyObj {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();

        ready(Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body)))
    }
}

async fn index() -> impl Responder {
    MyObj { name: "user" }
}

#[get("/stream")]
async fn stream() -> HttpResponse {
    let body = once(ok::<_, Error>(web::Bytes::from_static(b"test")));

    HttpResponse::Ok()
        .content_type("application/json")
        .streaming(body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(||
        App::new()
            .service(stream)
            .route("/index", web::get().to(index))
    )
    .bind("127.0.0.1:8080")?
    .run()
    .await
}