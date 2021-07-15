use actix_files::NamedFile;
use actix_session::Session;
use actix_web::middleware::errhandlers::ErrorHandlerResponse;
use actix_web::{dev, error, http, web, Error, HttpResponse, Result};
use serde::Deserialize;
use tera::{Context, Tera};

use crate::db;
use crate::session::{self, FlashMessage};

pub async fn index(
    pool: web::Data<db::PgPool>,
    tmpl: web::Data<Tera>,
    session: Session,
) -> Result<HttpResponse, Error> {
    let tasks = web::block(move || db::get_all_tasks(&pool)).await?;

    let mut context = Context::new();
    context.insert("tasks", &tasks);

    if let Some(flash) = session::get_flash(&session)? {
        context.insert("msg", &(flash.kind, flash.message));
        session::clear_flash(&session);
    }

    let rendered = tmpl
        .render("index.html.tera", &context)
        .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().body(rendered))
}