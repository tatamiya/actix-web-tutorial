use actix_web::{
    dev::HttpResponseBuilder, error, get, web, App, Result, HttpRequest, HttpServer, HttpResponse, http::header, http::StatusCode,
};
use derive_more::{Display, Error};


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

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(|| {


        App::new()
            .service(
                web::resource("/custom_error")
                .route(web::get().to(custom_error)),
            )
            .service(override_error_response)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
