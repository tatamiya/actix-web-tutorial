use actix_files as fs;
use actix_web::{get, HttpRequest, Result, Error};
use actix_web::http::header::{ContentDisposition, DispositionType};
use std::path::PathBuf;

async fn individual_file(req: HttpRequest) -> Result<fs::NamedFile> {
    let path: PathBuf = req.match_info().query("filename").parse().unwrap();
    Ok(fs::NamedFile::open(path)?)
}

#[get("/configuration/{filename:.*}")]
async fn configuration(req: HttpRequest) -> Result<fs::NamedFile, Error> {
    let path: PathBuf = req.match_info().query("filename").parse().unwrap();
    let file = fs::NamedFile::open(path)?;
    Ok(file
        .use_last_modified(true)
        .set_content_disposition(ContentDisposition {
            disposition: DispositionType::Attachment,
            parameters: vec![],
        })
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    use actix_web::{web, App, HttpServer};

    HttpServer::new(||
        App::new()
            .route("/individual_file/{filename:.*}", web::get().to(individual_file))
            .service(
                fs::Files::new("/static", ".")
                    .show_files_listing()
                    .use_last_modified(true),
            )
            //.service(fs::Files::new("/static", ".").index_file("index.html"))
            .service(configuration)
    )
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
