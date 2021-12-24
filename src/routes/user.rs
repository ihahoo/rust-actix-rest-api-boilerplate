use actix_web::{web};
use crate::api::user;

pub fn route(cfg: &mut web::ServiceConfig) {
    cfg.service(user::controller::get_info);
    cfg.service(user::controller::change_password);
}