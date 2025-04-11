use crate::lib::{client::ClientInfo, error};
use chrono::{DateTime, Utc};
use super::{AuthBlacklist, Authorization};

// 添加日志
pub async fn insert_log(log_type: i16, msg: &str, user_id: i32, auth_id: i32, client: &ClientInfo, log_time: DateTime<Utc>, db: &sqlx::Pool<sqlx::MySql>, log: &slog::Logger) -> Result<(), error::Error> {
    let r = sqlx::query(r#"
        INSERT INTO authorizations_logs (user_id, log_type, ip, log_time, client_type, auth_id, log, user_agent)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?)"#)
        .bind(user_id)
        .bind(log_type)
        .bind(&client.ip)
        .bind(log_time)
        .bind(10)
        .bind(auth_id)
        .bind(msg)
        .bind(&client.user_agent)
        .execute(db)
        .await;
    
    if let Err(err) = r {
        error!(log, "{}", err);
        return Err(error::err500());
    }

    Ok(())
}

// 将用户登录的token加入黑名单
pub async fn insert_auth_black_list(auth_black_list: &AuthBlacklist, db: &sqlx::Pool<sqlx::MySql>, log: &slog::Logger) -> Result<(), error::Error> {
    let r = sqlx::query(r#"
        INSERT INTO authorizations_blacklist (access_token_id, access_token_exp, user_id)
	    VALUES (?, ?, ?)"#)
        .bind(&auth_black_list.access_token_id)
        .bind(&auth_black_list.access_token_exp)
        .bind(&auth_black_list.user_id)
        .execute(db)
        .await;
    
    if let Err(err) = r {
        error!(log, "{}", err);
        return Err(error::err500());
    }

    Ok(())
}

// 插入授权
pub async fn insert_auth(authorization: &Authorization, db: &sqlx::Pool<sqlx::MySql>, log: &slog::Logger) -> Result<i32, error::Error> {
    // MySQL不支持RETURNING子句，需要先插入再查询
    let r = sqlx::query(r#"
        INSERT INTO authorizations (user_id, uuid, client_type, refresh_token, create_time, access_token_id, access_token_exp, access_token_iat, is_enabled)
	    VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"#)
        .bind(&authorization.user_id)
        .bind(&authorization.uuid)
        .bind(&authorization.client_type)
        .bind(&authorization.refresh_token)
        .bind(&authorization.create_time)
        .bind(&authorization.access_token_id)
        .bind(&authorization.access_token_exp)
        .bind(&authorization.access_token_iat)
        .bind(&authorization.is_enabled)
        .execute(db)
        .await;
    
    if let Err(e) = r {
        error!(log, "{}", e);
        return Err(error::err500());
    }
    
    // 获取最后插入的ID
    let result = r.unwrap();
    let last_id = result.last_insert_id();
    
    if last_id == 0 {
        error!(log, "Failed to get last insert ID");
        return Err(error::err500());
    }
    
    Ok(last_id as i32)
}


// 禁用授权
pub async fn disable_auth(id: i32, db: &sqlx::Pool<sqlx::MySql>, log: &slog::Logger) -> Result<(), error::Error> {
    let r = sqlx::query("UPDATE authorizations SET is_enabled=0, update_time=? WHERE id=?")
        .bind(Utc::now())
        .bind(id)
        .execute(db)
        .await;
    
    if let Err(err) = r {
        error!(log, "{}", err);
        return Err(error::err500());
    }

    Ok(())
}

// 通过id获取授权信息
pub async fn get_by_id(id: i32, db: &sqlx::Pool<sqlx::MySql>, log: &slog::Logger) -> Result<Option<Authorization>, error::Error> {
    let r = sqlx::query_as::<_, Authorization>(r#"
        SELECT a.*
        FROM authorizations a INNER JOIN users b ON a.user_id=b.id
        WHERE a.id=? AND b.is_enabled=1 AND b.is_del=0"#)
        .bind(id)
        .fetch_optional(db)
        .await;
    
    match r {
        Ok(v) => Ok(v),
        Err(e) => {
            error!(log, "{}", e);
            Err(error::err500())
        }
    }
}

// 通过uuid获取授权信息
pub async fn get_by_uuid(uuid: uuid::Uuid, db: &sqlx::Pool<sqlx::MySql>, log: &slog::Logger) -> Result<Option<Authorization>, error::Error> {
    let r = sqlx::query_as::<_, Authorization>("SELECT * FROM authorizations WHERE uuid=?")
        .bind(uuid.to_string())
        .fetch_optional(db)
        .await;
    
    match r {
        Ok(v) => Ok(v),
        Err(e) => {
            error!(log, "{}", e);
            Err(error::err500())
        }
    }
}

// 更新授权
pub async fn update_auth(authorization: &Authorization, db: &sqlx::Pool<sqlx::MySql>, log: &slog::Logger) -> Result<(), error::Error> {
    let id = match authorization.id {
        Some(v) => v,
        None => {
            error!(log, "update id error: {:?}", authorization.id);
            return Err(error::err500());
        }
    };

    let mut query = String::from("UPDATE authorizations SET ");
    let mut params: Vec<String> = Vec::new();

    // 固定更新字段
    params.push("update_time = ?".to_string());

    // 可选更新字段
    if authorization.refresh_token.is_some() {
        params.push("refresh_token = ?".to_string());
    }
    if authorization.last_refresh_time.is_some() {
        params.push("last_refresh_time = ?".to_string());
    }
    if authorization.access_token_id.is_some() {
        params.push("access_token_id = ?".to_string());
    }
    if authorization.access_token_exp.is_some() {
        params.push("access_token_exp = ?".to_string());
    }
    if authorization.access_token_iat.is_some() {
        params.push("access_token_iat = ?".to_string());
    }

    if params.is_empty() {
        return Ok(());
    }

    query.push_str(&params.join(", "));
    query.push_str(" WHERE id = ?");

    // 按顺序绑定参数
    let mut q = sqlx::query(&query)
        .bind(Utc::now());  // 绑定 update_time

    if let Some(refresh_token) = &authorization.refresh_token {
        q = q.bind(refresh_token);
    }
    if let Some(last_refresh_time) = authorization.last_refresh_time {
        q = q.bind(last_refresh_time);
    }
    if let Some(access_token_id) = &authorization.access_token_id {
        q = q.bind(access_token_id);
    }
    if let Some(access_token_exp) = authorization.access_token_exp {
        q = q.bind(access_token_exp);
    }
    if let Some(access_token_iat) = authorization.access_token_iat {
        q = q.bind(access_token_iat);
    }

    q = q.bind(id);

    let r = q.execute(db).await;

    if let Err(e) = r {
        error!(log, "{}", e);
        return Err(error::err500());
    }

    Ok(())
}