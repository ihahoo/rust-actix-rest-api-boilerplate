use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
use chrono::{Duration, Utc};
use super::{aes};
use chrono::prelude::*;
use crate::api::authorizations;
use actix_web::{web, HttpRequest};
use crate::AppState;
use crate::lib::client::ClientInfo;
use crate::lib::error;
use crate::api::authorizations::AuthorizationInfo;

const AES_KEY: &str = "e3Ui2PBkyFl5vUaO";
const JWT_KEY: &str = "TmeAdY8DIvUaJTkcMaVpJ8dUjIXN6qHosyGTULWhlVXfEvH6XKnDY1HzGVH64y00";

pub fn salt() -> uuid::Uuid {
    uuid::Uuid::new_v4()
}

pub fn crypt_password(password: &str, salt: &uuid::Uuid) -> String {
    let pwd = format!("{}{}", password, salt.to_string());
    let pwd = md5::compute(pwd);
    let pwd = format!("{:?}{}{}", pwd, password, salt.to_string());
    let pwd = Sha256::new().chain_update(pwd).finalize();
    format!("{:x}", pwd)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
    pub jti: String,
    pub scopes: Vec<String>,
}

#[derive(Debug)]
pub struct Token {
    pub token: String,
    pub expire_time: DateTime<Utc>,
    pub create_time: DateTime<Utc>,
    pub expire: i64,
    pub jti: uuid::Uuid,
}

#[derive(Debug)]
pub struct Auth {
    pub access_token: Token,
    pub refresh_token: Token,
    pub refresh_token_id: uuid::Uuid,
    pub auth_id: i32,
}

pub async fn create_auth(user_id: i32, user_type: i16, client: &ClientInfo, state: &web::Data<AppState>) -> Result<Auth, error::Error> {
    let access_token = create_access_token(user_id, user_type, &state.config);

    let refresh_token_id = uuid::Uuid::new_v4();
    let refresh_token_jti = uuid::Uuid::new_v4();

    let authorization = authorizations::Authorization {
        id: None,
        user_id: Some(user_id),
        uuid: Some(refresh_token_id),
        client_type: Some(10),
        refresh_token: Some(refresh_token_jti),
        create_time: Some(Utc::now()),
        update_time: None,
        last_refresh_time: None,
        access_token_id: Some(access_token.jti),
        access_token_exp: Some(access_token.expire_time),
        access_token_iat: Some(access_token.create_time),
        is_enabled: Some(1),
    };

    let authorization_id = authorizations::service::create_auth(&authorization, client, state).await?;

    let refresh_token = create_refresh_token(authorization_id, refresh_token_jti, &state.config);
    
    let auth = Auth {
        access_token,
        refresh_token,
        refresh_token_id,
        auth_id: authorization_id,
    };

    Ok(auth)
}

pub fn create_access_token(user_id: i32, user_type: i16, config: &config::Config) -> Token {
    let expire = config.get::<i64>("auth.access_token_expire").unwrap();

    let mut scopes = vec![String::from("ROLE_MEMBER")];
    if user_type == 10 {
        scopes.push(String::from("ROLE_ADMIN"));
    }

    let create_time = Utc::now();
    let expire_time = Utc::now() + Duration::seconds(expire);
    let jti = uuid::Uuid::new_v4();
    let sub = aes::encrypt(&user_id.to_string(), AES_KEY);

    let claim = Claims {
        sub,
        iat: create_time.timestamp() as usize,
        exp: expire_time.timestamp() as usize,
        jti: jti.to_string(),
        scopes,
    };

    let token = encode(&Header::new(Algorithm::HS256), &claim, &EncodingKey::from_secret(JWT_KEY.as_ref())).unwrap();

    Token {
        token,
        expire_time,
        create_time,
        expire,
        jti,
    }
}

pub fn create_refresh_token(authorization_id: i32, refresh_token_jti: uuid::Uuid, config: &config::Config) -> Token {
    let expire = config.get::<i64>("auth.refresh_token_expire").unwrap();
    let scopes = vec![String::from("ROLE_REFRESH_TOKEN")];

    let create_time = Utc::now();
    let expire_time = Utc::now() + Duration::seconds(expire);
    let jti = refresh_token_jti;
    let sub = aes::encrypt(&authorization_id.to_string(), AES_KEY);

    let claim = Claims {
        sub,
        iat: create_time.timestamp() as usize,
        exp: expire_time.timestamp() as usize,
        jti: jti.to_string(),
        scopes,
    };

    let token = encode(&Header::new(Algorithm::HS256), &claim, &EncodingKey::from_secret(JWT_KEY.as_ref())).unwrap();

    Token {
        token,
        expire_time,
        create_time,
        expire,
        jti,
    }
}

pub fn parse_token(token: &str) -> Result<Claims, error::Error> {
    let token = match decode::<Claims>(&token, &DecodingKey::from_secret(JWT_KEY.as_ref()), &Validation::default()) {
        Ok(v) => v,
        Err(_) => return Err(error::new(100403, "Authentication failure", 401))
    };

    let mut claims = token.claims;
    if let Some(v) = aes::decrypt(&claims.sub, AES_KEY) {
        claims.sub = v;
    } else {
        return Err(error::new(100403, "Authentication failure", 401));
    }

    Ok(claims)
}

pub async fn verify(permission: &str, req: &HttpRequest, state: &web::Data<AppState>) -> Result<AuthorizationInfo, error::Error> {
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

    let claims = parse_token(&token)?;
    let user_id = match claims.sub.parse::<i32>() {
        Err(_) => return Err(error::new(100403, "Authentication failure", 401)),
        Ok(v) => v
    };

    let mut scopes: Vec<String> = Vec::new();
    let mut have_permission = false;
    for v in claims.scopes {
        if permission.len() > 0 && v == permission {
            have_permission = true;
        }

        scopes.push(v);
    }

    if permission.len() > 0 && !have_permission {
        return Err(error::new(100404, "No permission", 403));
    }

    match authorizations::service::is_in_black_list(&claims.jti, &state).await {
        Err(_) => return Err(error::new(100403, "Authentication failure", 401)),
        Ok(v) => {
            if v {
                return Err(error::new(100403, "Authentication failure", 401));
            }
        }
    };

    let authorization_info = AuthorizationInfo {
        id: user_id,
        scopes,
    };

    Ok(authorization_info)
}