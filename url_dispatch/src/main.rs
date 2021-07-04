use actix_web::{guard, get, web, middleware, http::header, http::Method, App, HttpRequest, HttpResponse, HttpServer, Result, Responder};
use serde::Deserialize;

async fn hello() -> HttpResponse {
    HttpResponse::Ok().body("Hello")
}


#[get("/show")]
async fn show_users() -> HttpResponse {
    HttpResponse::Ok().body("Show users")
}

#[get("/show/{id}")]
async fn user_detail(path: web::Path<(u32, )>) -> HttpResponse {
    HttpResponse::Ok().body(format!("User detail: {}", path.into_inner().0))
}

#[get("/match_information/{v1}/{v2}")]
async fn match_information(req: HttpRequest) -> Result<String> {
    let v1: u8 = req.match_info().get("v1").unwrap().parse().unwrap();
    let v2: u8 = req.match_info().query("v2").parse().unwrap();
    let (v3, v4): (u8, u8) = req.match_info().load().unwrap();
    Ok(format!("Values {} {} {} {}", v1, v2, v3, v4))
}

#[get("/path_information/{username}/{id}/index.html")]
async fn path_information(info: web::Path<(String, u32)>) -> Result<String> {
    let info = info.into_inner();
    Ok(format!("Welcome {}! id: {}", info.0, info.1))
}

#[derive(Deserialize)]
struct Info {
    username: String,
}

#[get("/path_information/{username}/index.html")]
async fn path_info_to_struct(info: web::Path<Info>) -> Result<String> {
    Ok(format!("Welcome {}!", info.username))
}

#[get("/generate_url")]
async fn generate_url(req: HttpRequest) -> Result<HttpResponse> {
    let url = req.url_for("for", &["1", "2", "3"])?;

    Ok(HttpResponse::Found()
        .header(header::LOCATION, url.as_str())
        .finish()
    )
}

#[get("/external_resource")]
async fn external_resource(req: HttpRequest) -> impl Responder {
    let url = req.url_for("youtube", &["oHg5SJYRHA0"]).unwrap();
    assert_eq!(url.as_str(), "https://youtube.com/watch/oHg5SJYRHA0");

    url.into_string()
}

async fn normalize_and_redirect() -> HttpResponse {
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
            .service(
                web::scope("/scope")
                    .service(show_users)
                    .service(user_detail)
            )
            .service(match_information)
            .service(path_information)
            .service(path_info_to_struct)
            .service(
                web::resource("/generate_url/{a}/{b}/{c}")
                    .name("foo")
                    .guard(guard::Get())
                    .to(|| HttpResponse::Ok()),
            )
            .service(generate_url)
            .service(external_resource)
            .external_resource("youtube", "https://youtube.com/watch/{video_id}")
            .wrap(middleware::NormalizePath::default())
            .route("/resource/", web::to(normalize_and_redirect))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
