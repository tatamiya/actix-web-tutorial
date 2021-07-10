use actix_files::NamedFile;
use actix_web::{HttpRequest, Result};
use std::path::PathBuf;

async fn individual_file(req: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = req.match_info().query("filename").parse().unwrap();
    Ok(NamedFile::open(path)?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    use actix_web::{web, App, HttpServer};

    HttpServer::new(||
        App::new()
            .route("/individual_file/{filename:.*}", web::get().to(individual_file))
    )
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
