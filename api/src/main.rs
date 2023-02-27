pub mod api_requests;
use crate::api_requests::{respond, users::Users, ApiRequest};
use actix_files::Files;
use actix_web::{web, App, HttpRequest, HttpServer, Responder};

async fn api(raw_req: HttpRequest, body: String) -> impl Responder {
    let basic = ApiRequest::basic(raw_req, body);

    if basic.error.is_some() {
        return respond(Err(basic.error.unwrap()));
    }

    match basic.model.as_str() {
        "users" => ApiRequest::<Users>::from_basic(basic).evaluate(),
        _ => respond(Err((format!("Unknown model \"{}\"", basic.model), 400))),
    }
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

    let server = HttpServer::new(move || {
        App::new()
            .service(web::resource(vec!["/api/{model}/{action}*", "/api/{model}"]).to(api))
            .service(
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
