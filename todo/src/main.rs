#[macro_use]
extern crate diesel;
#[macro_use]
extern crate log;

use std::{env, io};

use actix_files as fs;
use actix_session::CookieSession;
use actix_web::middleware::{errhandlers::ErrorHandlers, Logger};
use actix_web::{http, web, App, HttpServer};
use dotenv::dotenv;
use tera::Tera;

mod api;
mod db;
mod model;
mod schema;
mod session;

static SESSION_SIGNING_KEY: &[u8] = &[0; 32];

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    env::set_var("RUST_LOG", "actix_todo=debug,actix_web=info");
    env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = db::init_pool(&database_url).expect("Failed to create pool");

    let app = move || {
        debug!("Constructiong the App");

        let mut templates = match Tera::new("templates/**/*") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        templates.autoescape_on(vec!["tera"]);

        let session_store = CookieSession::signed(SESSION_SIGNING_KEY).secure(false);

        App::new()
            .data(templates)
            .data(pool.clone())
            .wrap(Logger::default())
            .wrap(session_store)
            .service(web::resource("/").route(web::get().to(api::index)))
            .service(fs::Files::new("/static", "static/"))

    };

    debug!("Starting server");
    HttpServer::new(app).bind("localhost:8080")?.run().await
}
