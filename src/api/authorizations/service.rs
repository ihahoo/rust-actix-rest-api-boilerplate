use actix_web::web;
use futures::future::{BoxFuture, join_all};
use crate::AppState;
use crate::{lib, lib::{client::ClientInfo, error}};
use crate::api::authorizations::model;
use chrono::prelude::*;
use super::{AuthBlacklist, Authorization};
use crate::api::user;

// 添加日志
pub async fn insert_log(log_type: i16, msg: &str, user_id: i32, auth_id: i32, client: &ClientInfo, state: &web::Data<AppState>) -> Result<(), error::Error> {
    model::insert_log(log_type, msg, user_id, auth_id, client, Utc::now(), &state.db, &state.log).await?;
    
    Ok(())
}

// 将用户登录的token加入黑名单
pub async fn add_black_list(auth_black_list: &AuthBlacklist, state: &web::Data<AppState>) -> Result<(), error::Error> {
    let task1 = model::insert_auth_black_list(&auth_black_list, &state.db, &state.log);
    let mut hold: Vec<BoxFuture<_>> = vec![Box::pin(task1)];

    let diff = (auth_black_list.access_token_exp.time() - Utc::now().time()).num_seconds();
    if diff > 0 {
        let task2 = lib::redis::set_with_expire(
            format!("auth_black_list_{}", auth_black_list.access_token_id),
            auth_black_list.user_id,
            diff,
            &state.redis,
            &state.log
        );
        hold.push(Box::pin(task2));
    }

    join_all(hold).await;

    Ok(())
}

// 检查id是否在黑名单中
pub async fn is_in_black_list(id: &String, state: &web::Data<AppState>) -> Result<bool, error::Error> {
    let result = lib::redis::has_key(format!("auth_black_list_{}", id), &state.redis, &state.log).await?;

    Ok(result)
}

// 创建授权
pub async fn create_auth(authorization: &Authorization, client: &ClientInfo, state: &web::Data<AppState>) -> Result<i32, error::Error> {
    let result = model::insert_auth(authorization, &state.db, &state.log).await?;
    if let Some(user_id) = authorization.user_id {
        user::service::update_last_login(Utc::now(), user_id, &client, &state).await?;
    }
    if let Some(v) = result.id {
        return Ok(v);
    }

    Ok(0)
}

// 撤销授权
pub async fn revoke_auth(id: i32, state: &web::Data<AppState>) -> Result<(), error::Error> {
    model::disable_auth(id, &state.db, &state.log).await?;

    Ok(())
}

// 通过id获取授权信息
pub async fn get_by_id(id: i32, state: &web::Data<AppState>) -> Result<Option<Authorization>, error::Error> {
    let result = model::get_by_id(id, &state.db, &state.log).await?;

    Ok(result)
}

// 通过uuid获取授权信息
pub async fn get_by_uuid(uuid: &str, state: &web::Data<AppState>) -> Result<Option<Authorization>, error::Error> {
    let uid = match uuid::Uuid::parse_str(uuid) {
        Err(_) => return Err(error::new(100403, "Authentication failure", 401)),
        Ok(v) => v
    };
    
    let result = model::get_by_uuid(uid, &state.db, &state.log).await?;

    Ok(result)
}

// 更新授权
pub async fn update_auth(authorization: &Authorization, state: &web::Data<AppState>) -> Result<Authorization, error::Error> {
    let result = model::update_auth(authorization, &state.db, &state.log).await?;

    Ok(result)
}