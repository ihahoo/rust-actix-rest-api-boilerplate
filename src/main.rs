pub mod api;
pub mod lib;
mod routes;

use actix_cors::Cors;
use actix_web::middleware::errhandlers::{ErrorHandlers};
use actix_web::{http, web, App, HttpServer, Result, HttpResponse};
use lib::error;
use routes::hello;

#[derive(Clone)]
pub struct AppState {
    pub config: config::Config,
}

async fn index() -> Result<web::HttpResponse, error::Error> {
    Ok(HttpResponse::Ok().body(""))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut settings = config::Config::default();
    settings.merge(config::File::with_name("data/config/app.toml")).unwrap();
    let port = settings.get::<String>("app.port").unwrap();

    HttpServer::new(move || {
        let cors = Cors::permissive();

        println!("==> 🚀 {} listening at {}", "app", settings.get::<String>("app.port").unwrap());

        App::new()
            .data(AppState {
                config: settings.clone(),
            })
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
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}
