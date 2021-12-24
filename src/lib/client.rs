use actix_web::{web, HttpRequest, dev::ConnectionInfo};
use crate::AppState;

#[derive(Debug)]
pub struct ClientInfo {
    pub ip: String,
    pub user_agent: String,
}

pub fn get_client_info(state: &web::Data<AppState>, req: &HttpRequest, conn: &ConnectionInfo) -> ClientInfo {
    let mut ip = String::from("");
    let mut user_agent = String::from("");

    let is_behind_proxy = state.config.get::<bool>("app.behind_proxy").unwrap();
    if is_behind_proxy {
        if let Some(val) = conn.realip_remote_addr() {
            let split = val.split(":");
            let vec: Vec<&str> = split.collect();
            if vec.len() >0 {
                ip = vec[0].to_string();
            }
        }
    } else {
        if let Some(val) = req.peer_addr() {
            ip = val.ip().to_string();
        };
    }

    if let Some(val) = req.headers().get("User-Agent") {
        user_agent = val.to_str().unwrap_or_default().to_string();
    };
    
    ClientInfo { ip, user_agent }
}
