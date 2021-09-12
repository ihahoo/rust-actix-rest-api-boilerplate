use actix_web::{get, HttpResponse, Responder};
use serde::{Serialize};

#[derive(Serialize)]
struct Hello {
    msg: String,
}

#[get("/hello")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().json(Hello {msg: String::from("hello world!")})
}
