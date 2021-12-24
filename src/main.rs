pub mod api;
pub mod lib;
mod routes;

#[macro_use]
extern crate slog;
extern crate slog_term;
extern crate slog_async;
extern crate slog_json;

use actix_cors::Cors;
use actix_web::middleware::ErrorHandlers;
use actix_web::{http, web, App, HttpServer, Result, HttpResponse};
use lib::error;
use routes::{hello, authorizations, user};

#[derive(Clone)]
pub struct AppState {
    pub config: config::Config,
    pub log: slog::Logger,
    pub db: sqlx::Pool<sqlx::Postgres>,
    pub redis: mobc::Pool<lib::redis::RedisConnectionManager>
}

async fn index() -> Result<web::HttpResponse, error::Error> {
    Ok(HttpResponse::Ok().body(""))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // config
    let mut settings = config::Config::default();
    settings.merge(config::File::with_name("data/config/app.toml")).unwrap();
    let port = settings.get::<String>("app.port").unwrap();

    // log
    let logger = lib::log::get_logger();
    info!(logger, "==> 🚀 {} listening at {}", settings.get::<String>("app.name").unwrap(), settings.get::<String>("app.port").unwrap());

    // database
    let db_pool = lib::db::pg::conn(&settings).await;

    // redis
    let redis_pool = lib::redis::conn(&settings).await;

    HttpServer::new(move || {
        let cors = Cors::permissive();

        println!("==> 🚀 {} listening at {}", settings.get::<String>("app.name").unwrap(), settings.get::<String>("app.port").unwrap());

        App::new()
            .app_data(web::Data::new(AppState {
                config: settings.clone(),
                log: logger.clone(),
                db: db_pool.clone(),
                redis: redis_pool.clone(),
            }))
            .wrap(
                ErrorHandlers::new()
                    .handler(http::StatusCode::METHOD_NOT_ALLOWED, error::render_405)
                    .handler(http::StatusCode::NOT_FOUND, error::render_404)
                    .handler(http::StatusCode::INTERNAL_SERVER_ERROR, error::render_500)
                    .handler(http::StatusCode::BAD_REQUEST, error::render_400),
            )
            .wrap(cors)
            .configure(hello::route)
            .configure(authorizations::route)
            .configure(user::route)
            .service(web::resource("/").route(web::get().to(index)))
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}
