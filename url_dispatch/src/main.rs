use actix_web::{guard, get, web, App, HttpRequest, HttpResponse, HttpServer, Result};

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
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
