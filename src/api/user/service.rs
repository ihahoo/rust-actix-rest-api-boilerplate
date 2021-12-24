use actix_web::web;
use crate::AppState;
use crate::lib::{client::ClientInfo, error};
use super::{model, User, UserInfo};
use chrono::prelude::*;

pub async fn get_by_id(id: i32, state: &web::Data<AppState>) -> Result<Option<User>, error::Error> {
    let result = model::get_by_id(id, &state.db, &state.log).await?;

    Ok(result)
}

pub async fn get_by_username(username: &str, state: &web::Data<AppState>) -> Result<Option<User>, error::Error> {
    let result = model::get_by_username(&username, &state.db, &state.log).await?;

    Ok(result)
}

pub async fn insert(user: &User, state: &web::Data<AppState>) -> Result<User, error::Error> {
    let result = model::insert(user, &state.db, &state.log).await?;

    Ok(result)
}

pub async fn update(user: &User, state: &web::Data<AppState>) -> Result<User, error::Error> {
    let result = model::update(user, &state.db, &state.log).await?;

    Ok(result)
}

pub async fn delete(id: i32, state: &web::Data<AppState>) -> Result<(), error::Error> {
    model::delete(id, &state.db, &state.log).await?;

    Ok(())
}

pub async fn update_last_login(login_time: DateTime<Utc>, user_id: i32, client: &ClientInfo, state: &web::Data<AppState>) -> Result<(), error::Error> {
    model::update_last_login(login_time, &client.ip, user_id, &state.db, &state.log).await?;

    Ok(())
}

pub async fn get_user_info_by_id(id: i32, state: &web::Data<AppState>) -> Result<Option<UserInfo>, error::Error> {
    let result = model::get_user_info_by_id(id, &state.db, &state.log).await?;

    Ok(result)
}
