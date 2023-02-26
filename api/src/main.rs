pub mod api_requests;
use crate::api_requests::{users::Users, ApiRequest};
use actix_files::Files;
use actix_web::{route, App, HttpRequest, HttpResponse, HttpServer, Responder};
#[route(
    "/api/{model}/{action}*",
    method = "GET",
    method = "POST",
    method = "DELETE",
    method = "PUT"
)]
async fn api(raw_req: HttpRequest) -> impl Responder {
    let basic = ApiRequest::basic(raw_req);
    // log timestamp, method, path, model, action
    println!(
        "{} {} {} {} {}",
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
        basic.method,
        basic.path,
        basic.model,
        basic.action
    );
    if basic.error.is_some() {
        return HttpResponse::BadRequest().body(basic.error.unwrap());
    }
    let req = match basic.model.as_str() {
        "users" => ApiRequest::<Users>::from_basic(basic),
        _ => return HttpResponse::BadRequest().body(format!("Unknown model \"{}\"", basic.model)),
    };
    req.evaluate()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server");
    let public_dir = std::env::var("public_dir").unwrap_or_else(|_| "../app/public".into());
    let port = std::env::var("port")
        .unwrap_or_else(|_| "8080".into())
        .parse::<u16>()
        .expect("Expected port to be a number");
    let host = std::env::var("host").unwrap_or_else(|_| "127.0.0.1".into());
    println!("Public dir: {}", public_dir);
    // list the files in the public dir
    let mut files = std::fs::read_dir(&public_dir).unwrap();
    while let Some(Ok(file)) = files.next() {
        println!("File: {}", file.path().display());
    }
    let server = HttpServer::new(move || {
        App::new().service(api).service(
            Files::new("/", public_dir.as_str())
                .prefer_utf8(true)
                .show_files_listing()
                .index_file("index.html")
                .use_last_modified(true),
        )
    })
    .bind((host, port))?
    .run();
    println!("Server started");
    server.await
}
