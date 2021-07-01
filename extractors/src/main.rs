use actix_web::{error, get, post, web, Result, HttpRequest, HttpResponse, Responder};
use serde::Deserialize;
use std::cell::Cell;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

#[get("/users/{user_id}/{friend}")]
async fn index(web::Path((user_id, friend)): web::Path<(u32, String)>) -> Result<String> {
    Ok(format!("Welcome {}, user_id {}!", friend, user_id))
}

#[derive(Deserialize)]
struct Info {
    user_id: u32,
    friend: String,
}

#[get("/users2/{user_id}/{friend}")]
async fn index2(info: web::Path<Info>) -> Result<String> {
    Ok(format!(
        "Welcome {}, user_id {}",
        info.friend, info.user_id
    ))
}

#[get("/users3/{userid}/{friend}")]
async fn index3(req: HttpRequest) -> Result<String> {
    let name: String = req.match_info().get("friend").unwrap().parse().unwrap();
    let userid: i32 = req.match_info().query("userid").parse().unwrap();

    Ok(format!("Welcome {}, userid {}!", name, userid))
}

#[derive(Deserialize)]
struct Info2 {
    username: String,
}

// http://127.0.0.1:8080/query?username=hoge
#[get("/query")]
async fn query(info: web::Query<Info2>) -> String {
    format!("Welcome {}!", info.username)
}

#[get("/json")]
async fn json(info: web::Json<Info2>) -> Result<String> {
    Ok(format!("Wolcome {}!", info.username))
}

async fn json2(info: web::Json<Info2>) -> impl Responder {
    format!("Welcome {}!", info.username)
}

#[derive(Deserialize)]
struct FormData {
    username: String,
}

#[post("/form")]
async fn form_data(form: web::Form<FormData>) -> Result<String> {
    Ok(format!("Welcome {}!", form.username))
}

#[derive(Clone)]
struct AppState {
    local_count: Cell<usize>,
    global_count: Arc<AtomicUsize>,
}

async fn show_count(data: web::Data<AppState>) -> impl Responder {
    format!(
        "global_count: {}\nlocal_count: {}",
        data.global_count.load(Ordering::Relaxed),
        data.local_count.get()
    )
}

async fn add_one(data: web::Data<AppState>) -> impl Responder {
    data.global_count.fetch_add(1, Ordering::Relaxed);

    let local_count = data.local_count.get();
    data.local_count.set(local_count + 1);

    format!(
        "global_count: {}\nlocal_count: {}",
        data.global_count.load(Ordering::Relaxed),
        data.local_count.get()
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};

    HttpServer::new(|| {
        let json_config = web::JsonConfig::default()
            .limit(4096)
            .error_handler(|err, _req| {
                error::InternalError::from_response(err, HttpResponse::Conflict().finish()).into()
            });

        let data = AppState {
            local_count: Cell::new(0),
            global_count: Arc::new(AtomicUsize::new(0)),
        };

        App::new()
            .service(index)
            .service(index2)
            .service(index3)
            .service(query)
            .service(json)
            .service(
                web::resource("/json2")
                .app_data(json_config)
                .route(web::post().to(json2)),
            )
            .service(form_data)
            .data(data.clone())
            .route("/show_count", web::to(show_count))
            .route("/add", web::to(add_one))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
