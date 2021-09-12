use actix_web::{web};
use crate::api::hello::hello;

pub fn route(cfg: &mut web::ServiceConfig) {
    cfg.service(hello);
}
