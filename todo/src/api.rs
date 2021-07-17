use actix_files::NamedFile;
use actix_session::Session;
use actix_web::middleware::errhandlers::ErrorHandlerResponse;
use actix_web::{dev, error, http, web, Error, HttpResponse, Result};
use serde::Deserialize;
use tera::{Context, Tera};

use crate::db;
use crate::session::{self, FlashMessage};

pub async fn index(
    pool: web::Data<db::SqlitePool>,
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

#[derive(Deserialize)]
pub struct CreateForm {
    description: String,
}

pub async fn create(
    params: web::Form<CreateForm>,
    pool: web::Data<db::SqlitePool>,
    session: Session,
) -> Result<HttpResponse, Error> {
    if params.description.is_empty() {
        session::set_flash(
            &session,
            FlashMessage::error("Description cannot be empty"),
        )?;
        Ok(redirect_to("/"))
    } else {
        web::block(move || db::create_task(params.into_inner().description, &pool))
            .await?;
        session::set_flash(&session, FlashMessage::success("Task successfully added"))?;
        Ok(redirect_to("/"))
    }
}

fn redirect_to(location: &str) -> HttpResponse {
    HttpResponse::Found()
        .header(http::header::LOCATION, location)
        .finish()
}

pub fn bad_request<B>(res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    let new_resp = NamedFile::open("static/errors/400.html")?
        .set_status_code(res.status())
        .into_response(res.request())?;
    Ok(ErrorHandlerResponse::Response(
        res.into_response(new_resp.into_body()),
    ))
}

pub fn not_found<B>(res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    let new_resp = NamedFile::open("static/errors/404.html")?
        .set_status_code(res.status())
        .into_response(res.request())?;
    Ok(ErrorHandlerResponse::Response(
        res.into_response(new_resp.into_body()),
    ))
}

pub fn internal_server_error<B>(
    res: dev::ServiceResponse<B>,
) -> Result<ErrorHandlerResponse<B>> {
    let new_resp = NamedFile::open("static/errors/500.html")?
        .set_status_code(res.status())
        .into_response(res.request())?;
    Ok(ErrorHandlerResponse::Response(
        res.into_response(new_resp.into_body()),
    ))
}