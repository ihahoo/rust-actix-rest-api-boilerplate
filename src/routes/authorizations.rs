use actix_web::{web};
use crate::api::authorizations;

pub fn route(cfg: &mut web::ServiceConfig) {
    cfg.service(authorizations::controller::create_auth);
    cfg.service(authorizations::controller::refresh_auth);
    cfg.service(authorizations::controller::delete_auth);
}