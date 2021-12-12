use actix_web::{web, get, HttpResponse, Responder};
use serde::{Serialize};
use crate::AppState;

#[derive(Serialize)]
struct Hello {
    msg: String,
}

#[get("/hello")]
pub async fn hello(state: web::Data<AppState>) -> impl Responder {
    let name = state.config.get::<String>("app.name").unwrap();
    info!(state.log, "hello {}", name);
    HttpResponse::Ok().json(Hello {msg: format!("hello {}", name)})
}
