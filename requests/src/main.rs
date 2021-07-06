use std::io::Write;

use actix_web::{error, middleware, web, get, post, App, HttpServer, HttpResponse, Result, Error};
use actix_multipart::Multipart;
use futures::{StreamExt, TryStreamExt};
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

async fn save_file(mut payload: Multipart) -> Result<HttpResponse, Error> {
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field.content_disposition().unwrap();
        let filename = content_type.get_filename().unwrap();
        let filepath = format!("./tmp/{}", sanitize_filename::sanitize(&filename));

        let mut f = web::block(|| std::fs::File::create(filepath))
            .await
            .unwrap();

        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            f = web::block(move || f.write_all(&data).map(|_| f)).await?;
        }
    }

    Ok(HttpResponse::Ok().into())
}

fn index() -> HttpResponse {
    let html = r#"<html>
        <head><title>Upload Test</title></head>
        <body>
            <form target="/" method="post" enctype="multipart/form-data">
                <input type="file" multiple name="file"/>
                <button type="submit">Submit</button>
            </form>
        </body>
    </html>"#;

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    std::fs::create_dir_all("./tmp").unwrap();

    HttpServer::new(||
        App::new()
            .route("/json_request", web::post().to(json_request))
            .service(load_and_deserialize)
            .service(url_encoded_body)
            .service(streaming_request)
            .wrap(middleware::Logger::default())
            .service(
                web::resource("/multipart")
                    .route(web::get().to(index))
                    .route(web::post().to(save_file))
            )
    )
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
