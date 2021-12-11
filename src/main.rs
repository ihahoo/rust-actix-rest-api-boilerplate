pub mod api;
pub mod lib;
mod routes;

use actix_cors::Cors;
use actix_web::middleware::errhandlers::{ErrorHandlers};
use actix_web::{http, web, App, HttpServer, Result, HttpResponse};
use lib::error;
use routes::hello;

async fn index() -> Result<web::HttpResponse, error::Error> {
    Ok(HttpResponse::Ok().body(""))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::permissive();

        println!("==> 🚀 {} listening at {}", "app", "8080");

        App::new()
            .wrap(
                ErrorHandlers::new()
                    .handler(http::StatusCode::METHOD_NOT_ALLOWED, error::render_405)
                    .handler(http::StatusCode::NOT_FOUND, error::render_404)
                    .handler(http::StatusCode::INTERNAL_SERVER_ERROR, error::render_500)
                    .handler(http::StatusCode::BAD_REQUEST, error::render_400),
            )
            .wrap(cors)
            .configure(hello::route)
            .service(web::resource("/").route(web::get().to(index)))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
