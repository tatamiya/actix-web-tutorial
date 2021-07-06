use actix_web::{error, web, get, post, App, HttpServer, HttpResponse, Result, Error};
use futures::StreamExt;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Info {
    username: String,
}

async fn json_request(info: web::Json<Info>) -> Result<String> {
    Ok(format!("Welcome {}!", info.username))
}

#[derive(Serialize, Deserialize)]
struct MyObj {
    name: String,
    number: i32,
}

const MAX_SIZE: usize = 262_144;

#[post("/load_and_deserialize")]
async fn load_and_deserialize(mut payload: web::Payload) -> Result<HttpResponse, Error> {
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;

        if (body.len() + chunk.len()) > MAX_SIZE {
            return Err(error::ErrorBadRequest("overflow"));
        }
        body.extend_from_slice(&chunk);
    }

    let obj = serde_json::from_slice::<MyObj>(&body)?;
    Ok(HttpResponse::Ok().json(obj))
}

#[derive(Deserialize)]
struct FormData {
    username: String,
}

#[post("/url_encoded_body")]
async fn url_encoded_body(form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().body(format!("username: {}", form.username))
}

#[get("/streaming_request")]
async fn streaming_request(mut body: web::Payload) -> Result<HttpResponse> {
    let mut bytes = web::BytesMut::new();
    while let Some(item) = body.next().await {
        let item = item?;
        println!("Chunk: {:?}", &item);
        bytes.extend_from_slice(&item);
    }

    Ok(HttpResponse::Ok().finish())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(||
        App::new()
            .route("/json_request", web::post().to(json_request))
            .service(load_and_deserialize)
            .service(url_encoded_body)
            .service(streaming_request)
    )
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
