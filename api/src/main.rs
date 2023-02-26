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
    // let _public_dir = std::env::var("public_dir").unwrap_or_else(|_| "../app/public".into())
    let public_dir = "/www";
    HttpServer::new(move || {
        App::new().service(api).service(
            Files::new("/", public_dir)
                .prefer_utf8(true)
                .show_files_listing()
                .index_file("index.html")
                .use_last_modified(true),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
