pub mod controller;
pub mod model;
pub mod service;

use chrono::prelude::*;

#[derive(Debug)]
pub struct AuthBlacklist {
    pub id: Option<i32>,
    pub access_token_id: uuid::Uuid,
    pub access_token_exp: DateTime<Utc>,
    pub user_id: i32,
}

#[derive(Debug, sqlx::FromRow)]
pub struct Authorization {
    pub id: Option<i32>,
    pub user_id: Option<i32>,
    pub uuid: Option<uuid::Uuid>,
    pub client_type: Option<i16>,
    pub refresh_token: Option<uuid::Uuid>,
    pub create_time: Option<DateTime<Utc>>,
    pub update_time: Option<DateTime<Utc>>,
    pub last_refresh_time: Option<DateTime<Utc>>,
    pub access_token_id: Option<uuid::Uuid>,
    pub access_token_exp: Option<DateTime<Utc>>,
    pub access_token_iat: Option<DateTime<Utc>>,
    pub is_enabled: Option<i16>,
}

#[derive(Debug)]
pub struct AuthorizationInfo {
    pub id: i32,
    pub scopes: Vec<String>,
}