use actix_web::{get, web, App, HttpServer, HttpResponse, Responder};
use std::sync::Mutex;

async fn index() -> impl Responder {
    "Hello world!"
}

struct AppState {
    app_name: String,
}

#[get("/")]
async fn state(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name;
    format!("Hello {}!", app_name)
}

struct AppStateWithCounter {
    counter: Mutex<i32>,
}

async fn shared_mutable_state(data: web::Data<AppStateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;

    format!("Request number: {}", counter)
}

fn scoped_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/test")
            .route(web::get().to(|| HttpResponse::Ok().body("test")))
            .route(web::head().to(|| HttpResponse::MethodNotAllowed())),
    );
}

fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/hoge")
            .route(web::get().to(|| HttpResponse::Ok().body("hoge")))
            .route(web::head().to(|| HttpResponse::MethodNotAllowed())),
    );
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let counter = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0),
    });

    HttpServer::new(move || {
        App::new()
            .data(AppState {
                app_name: String::from("Actix-web"),
            })
            .service(
                web::scope("/app")
                .route("/index.html", web::get().to(index)),
            )
            .service(state)
            .app_data(counter.clone())
            .route("/shared_mutable_state.html", web::get().to(shared_mutable_state))
            .configure(config)
            .service(web::scope("/api").configure(scoped_config))
            .route("/", web::get().to(|| HttpResponse::Ok().body("/")))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}