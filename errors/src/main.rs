use actix_web::{
    dev::HttpResponseBuilder, error, get, web, middleware::Logger, App, Result, HttpRequest, HttpServer, HttpResponse, http::header, http::StatusCode,
};
use derive_more::{Display, Error};
use log ::info;


#[derive(Debug, Display, Error)]
#[display(fmt = "my error: {}", name)]
struct MyError {
    name: &'static str,
}

impl error::ResponseError for MyError {}

async fn custom_error() -> Result<&'static str, MyError> {
    Err(MyError { name: "test"})
}

#[derive(Debug, Display, Error)]
enum MyError2 {
    #[display(fmt = "internal error")]
    InternalError,

    #[display(fmt = "bad request")]
    BadClientData,

    #[display(fmt = "timeout")]
    Timeout,
}

impl error::ResponseError for MyError2 {
    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code())
            .set_header(header::CONTENT_TYPE, "text/html; charset=utf-8")
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            MyError2::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            MyError2::BadClientData => StatusCode::BAD_REQUEST,
            MyError2::Timeout => StatusCode::GATEWAY_TIMEOUT,
        }
    }
}

#[get("/override_error_response")]
async fn override_error_response() -> Result<&'static str, MyError2> {
    Err(MyError2::BadClientData)
}

#[derive(Debug)]
struct MyErrorWithHelper {
    name: &'static str,
}

#[get("/error_helpers")]
async fn error_helpers() -> Result<&'static str> {
    let result: Result<&'static str, MyErrorWithHelper> = Err(MyErrorWithHelper {name: "test error"});
    Ok(result.map_err(|e| error::ErrorBadRequest(e.name))?)
}

#[derive(Debug, Display, Error)]
enum UserError {
    #[display(fmt = "Validation error on field: {}", field)]
    ValidationError { field: String },

    #[display(fmt = "an internal error occurred. Please try again later.")]
    InternalError,
}

impl error::ResponseError for UserError {
    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code())
        .set_header(header::CONTENT_TYPE, "text/html; charset=utf-8")
        .body(self.to_string())
    }
    fn status_code(&self) -> StatusCode {
        match *self {
            UserError::ValidationError { .. } => StatusCode::BAD_REQUEST,
            UserError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[derive(Debug, Display,   Error)]
#[display(fmt = "my error: {}", name)]
pub struct MyErrorForLogging {
    name: &'static str,
}

impl error::ResponseError for MyErrorForLogging {}


#[get("/logging")]
async fn logging() -> Result<&'static str, MyErrorForLogging> {
    let err = MyErrorForLogging {name: "test error"};
    info!("{}", err);
    Err(err)
}

#[rustfmt::skip]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    HttpServer::new(|| {
        let logger = Logger::default();

        App::new()
            .service(
                web::resource("/custom_error")
                .route(web::get().to(custom_error)),
            )
            .service(override_error_response)
            .service(error_helpers)
            .wrap(logger)
            .service(logging)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
