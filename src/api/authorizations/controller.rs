use actix_web::{web, post, put, delete, HttpResponse, HttpRequest};
use actix_web::dev::ConnectionInfo;
use serde::{Serialize, Deserialize};
use crate::AppState;
use crate::lib::{error, validator, client, auth};
use crate::api::user;
use super::{service, AuthBlacklist, Authorization};
use chrono::prelude::*;

#[derive(Deserialize)]
pub struct CreateAuthReqJson {
    username: Option<String>,
    password: Option<String>,
}

#[derive(Serialize)]
struct ResTokenJson {
    id: String,
    access_token: String,
    expires_in: i64,
    refresh_token: String,
    created_at: String,
    updated_at: String,
}

// 创建授权
#[post("/authorizations")]
pub async fn create_auth(req_info: web::Json<CreateAuthReqJson>, state: web::Data<AppState>, req: HttpRequest, conn: ConnectionInfo) -> Result<HttpResponse, error::Error> {
    let username = validator::required_str(&req_info.username, "用户名")?;
    let password = validator::required_str(&req_info.password, "密码")?;

    let client = client::get_client_info(&state, &req, &conn);

    let result = user::service::get_by_username(&username, &state).await?;
    let u = match result {
        None => {
            service::insert_log(1003, &username, 0, 0, &client, &state).await?;
            return Err(error::new(100400, "帐号或密码不正确", 422));
        },
        Some(v) => v
    };

    let user_id = match u.id {
        None => return Err(error::new(100400, "帐号或密码不正确", 422)),
        Some(v) => v,
    };

    let user_type = match u.user_type {
        None => return Err(error::new(100400, "帐号或密码不正确", 422)),
        Some(v) => v,
    };

    let is_del = match u.is_del {
        None => return Err(error::new(100400, "帐号或密码不正确", 422)),
        Some(v) => v,
    };
    if is_del != 0 {
        service::insert_log(1004, "", user_id, 0, &client, &state).await?;
        return Err(error::new(100400, "帐号或密码不正确", 422));
    }

    let is_enabled = match u.is_enabled {
        None => return Err(error::new(100400, "帐号或密码不正确", 422)),
        Some(v) => v,
    };
    if is_enabled != 1 {
        service::insert_log(1002, "", user_id, 0, &client, &state).await?;
        return Err(error::new(100400, "帐号或密码不正确", 422));
    }

    let user_password = match u.password {
        None => return Err(error::new(100400, "帐号或密码不正确", 422)),
        Some(v) => v,
    };
    let salt = match u.salt {
        None => return Err(error::new(100400, "帐号或密码不正确", 422)),
        Some(v) => v,
    };
    let pwd = auth::crypt_password(&password, &salt);
    if user_password != pwd {
        service::insert_log(1001, "", user_id, 0, &client, &state).await?;
        return Err(error::new(100400, "帐号或密码不正确", 422));
    }

    let auth = auth::create_auth(user_id, user_type, &client, &state).await?;
    service::insert_log(1, "", user_id, auth.auth_id, &client, &state).await?;

    Ok(HttpResponse::Ok().json(ResTokenJson {
        id: auth.refresh_token_id.to_string(),
        access_token: auth.access_token.token,
        expires_in: auth.access_token.expire,
        refresh_token: auth.refresh_token.token,
        created_at: format!("{:?}", auth.access_token.create_time),
        updated_at: format!("{:?}", auth.access_token.create_time),
    }))
}

// 刷新授权
#[put("/authorizations/{id}")]
pub async fn refresh_auth(req: HttpRequest, state: web::Data<AppState>, conn: ConnectionInfo) -> Result<HttpResponse, error::Error> {
    let id: String = req.match_info().get("id").unwrap().parse().unwrap();
    validator::uuid(&id, "授权id")?;

    let token = match req.headers().get("Authorization") {
        None => return Err(error::new(100403, "Authentication failure", 401)),
        Some(v) => {
            if v.len() <= 6 {
                return Err(error::new(100403, "Authentication failure", 401));
            }
            let v = v.to_str().unwrap_or_default().to_string();
            if &v[..7] != "Bearer " {
                return Err(error::new(100403, "Authentication failure", 401));
            }
            String::from(&v[7..])
        }
    };
    
    let client = client::get_client_info(&state, &req, &conn);

    let claims = auth::parse_token(&token)?;
    let mut have_permission = false;
    for v in claims.scopes {
        if v == "ROLE_REFRESH_TOKEN" {
            have_permission = true;
            break;
        }
    }
    if !have_permission {
        service::insert_log(1053, "", 0, 0, &client, &state).await?;
        return Err(error::new(100404, "No permission", 403));
    }

    let auth_id = claims.sub.parse::<i32>().unwrap();

    let auth_data = match service::get_by_id(auth_id, &state).await? {
        None => {
            service::insert_log(1058, "", 0, auth_id, &client, &state).await?;
            return Err(error::new(100403, "Authentication failure", 401));
        },
        Some(v) => v
    };

    let user_id = match auth_data.user_id {
        None => return Err(error::new(100403, "Authentication failure", 401)),
        Some(v) => v,
    };

    let user_data = match user::service::get_by_id(user_id, &state).await? {
        None => {
            service::insert_log(1058, "", 0, auth_id, &client, &state).await?;
            return Err(error::new(100403, "Authentication failure", 401));
        },
        Some(v) => v
    };

    match auth_data.uuid {
        None => return Err(error::new(100403, "Authentication failure", 401)),
        Some(v) => {
            if v.to_string() != id {
                service::insert_log(1059, "", user_id, auth_id, &client, &state).await?;
                return Err(error::new(100403, "Authentication failure", 401));
            }
        },
    };

    match auth_data.is_enabled {
        None => return Err(error::new(100403, "Authentication failure", 401)),
        Some(v) => {
            if v != 1 {
                service::insert_log(1060, "", user_id, auth_id, &client, &state).await?;
                return Err(error::new(100403, "Authentication failure", 401));
            }
        },
    };

    match auth_data.refresh_token {
        None => return Err(error::new(100403, "Authentication failure", 401)),
        Some(v) => {
            if v.to_string() != claims.jti {
                service::insert_log(1060, "", user_id, auth_id, &client, &state).await?;
                return Err(error::new(100403, "Authentication failure", 401));
            }
        },
    };

    match user_data.is_enabled {
        None => return Err(error::new(100403, "Authentication failure", 401)),
        Some(v) => {
            if v != 1 {
                service::insert_log(1061, "", user_id, auth_id, &client, &state).await?;
                return Err(error::new(100403, "Authentication failure", 401));
            }
        },
    };

    match user_data.is_del {
        None => return Err(error::new(100403, "Authentication failure", 401)),
        Some(v) => {
            if v != 0 {
                service::insert_log(1062, "", user_id, auth_id, &client, &state).await?;
                return Err(error::new(100403, "Authentication failure", 401));
            }
        },
    };

    let user_type = match user_data.user_type {
        None => return Err(error::new(100403, "Authentication failure", 401)),
        Some(v) => v,
    };

    let access_token_id = match auth_data.access_token_id {
        None => return Err(error::new(100403, "Authentication failure", 401)),
        Some(v) => v,
    };

    let access_token_exp = match auth_data.access_token_exp {
        None => return Err(error::new(100403, "Authentication failure", 401)),
        Some(v) => v,
    };

    let create_time = match auth_data.create_time {
        None => return Err(error::new(100403, "Authentication failure", 401)),
        Some(v) => v,
    };

    let refresh_token_jti = uuid::Uuid::new_v4();
    let update_time = Utc::now();

    let access_token = auth::create_access_token(user_id, user_type, &state.config);
    let refresh_token = auth::create_refresh_token(auth_id, refresh_token_jti, &state.config);

    let authorization_blacklist = AuthBlacklist {
        id: None,
        access_token_id,
        access_token_exp,
        user_id,
    };
    
    service::add_black_list(&authorization_blacklist, &state).await?;

    let authorization = Authorization {
        id: Some(auth_id),
        user_id: None,
        uuid: None,
        client_type: None,
        refresh_token: Some(refresh_token_jti),
        create_time: None,
        update_time: Some(update_time),
        last_refresh_time: Some(update_time),
        access_token_id: Some(access_token.jti),
        access_token_exp: Some(access_token.expire_time),
        access_token_iat: Some(access_token.create_time),
        is_enabled: None,
    };

    service::update_auth(&authorization, &state).await?;
    service::insert_log(2, "", user_id, auth_id, &client, &state).await?;

    Ok(HttpResponse::Ok().json(ResTokenJson {
        id,
        access_token: access_token.token,
        expires_in: access_token.expire,
        refresh_token: refresh_token.token,
        created_at: format!("{:?}", create_time),
        updated_at: format!("{:?}", update_time),
    }))
}

// 删除授权
#[delete("/authorizations/{id}")]
pub async fn delete_auth(req: HttpRequest, state: web::Data<AppState>, conn: ConnectionInfo) -> Result<HttpResponse, error::Error> {
    let id: String = req.match_info().get("id").unwrap().parse().unwrap();
    validator::uuid(&id, "授权id")?;

    let client = client::get_client_info(&state, &req, &conn);
    
    let auth_data = service::get_by_uuid(&id, &state).await?;
    let auth_data = match auth_data {
        None => {
            service::insert_log(1101, "", 0, 0, &client, &state).await?;
            return Err(error::new(100403, "Authentication failure", 401));
        },
        Some(v) => v
    };

    let user_id = match auth_data.user_id {
        None => return Err(error::new(100403, "Authentication failure", 401)),
        Some(v) => v,
    };

    let auth_id = match auth_data.id {
        None => return Err(error::new(100403, "Authentication failure", 401)),
        Some(v) => v,
    };

    match auth_data.is_enabled {
        None => return Err(error::new(100403, "Authentication failure", 401)),
        Some(v) => {
            if v != 1 {
                service::insert_log(1102, "", user_id, auth_id, &client, &state).await?;
                return Err(error::new(100403, "Authentication failure", 401));
            }
        },
    };

    let access_token_id = match auth_data.access_token_id {
        None => return Err(error::new(100403, "Authentication failure", 401)),
        Some(v) => v,
    };

    let access_token_exp = match auth_data.access_token_exp {
        None => return Err(error::new(100403, "Authentication failure", 401)),
        Some(v) => v,
    };

    service::revoke_auth(auth_id, &state).await?;

    let authorization_blacklist = AuthBlacklist {
        id: None,
        access_token_id,
        access_token_exp,
        user_id,
    };
    
    service::add_black_list(&authorization_blacklist, &state).await?;

    service::insert_log(3, "", user_id, auth_id, &client, &state).await?;

    Ok(HttpResponse::Ok().body(""))
}
