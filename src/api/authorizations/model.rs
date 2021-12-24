use crate::lib::{client::ClientInfo, error};
use chrono::{DateTime, Utc};
use super::{AuthBlacklist, Authorization};

// 添加日志
pub async fn insert_log(log_type: i16, msg: &str, user_id: i32, auth_id: i32, client: &ClientInfo, log_time: DateTime<Utc>, db: &sqlx::Pool<sqlx::Postgres>, log: &slog::Logger) -> Result<(), error::Error> {
    let r = sqlx::query(r#"
        INSERT INTO authorizations_logs (user_id, log_type, ip, log_time, client_type, auth_id, log, user_agent)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"#)
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
pub async fn insert_auth_black_list(auth_black_list: &AuthBlacklist, db: &sqlx::Pool<sqlx::Postgres>, log: &slog::Logger) -> Result<(), error::Error> {
    let r = sqlx::query(r#"
        INSERT INTO authorizations_blacklist (access_token_id, access_token_exp, user_id)
	    VALUES ($1, $2, $3)"#)
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
pub async fn insert_auth(authorization: &Authorization, db: &sqlx::Pool<sqlx::Postgres>, log: &slog::Logger) -> Result<Authorization, error::Error> {
    let r = sqlx::query_as::<_, Authorization>(r#"
        INSERT INTO authorizations (user_id, uuid, client_type, refresh_token, create_time, access_token_id, access_token_exp, access_token_iat, is_enabled)
	    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
	    RETURNING *"#)
        .bind(&authorization.user_id)
        .bind(&authorization.uuid)
        .bind(&authorization.client_type)
        .bind(&authorization.refresh_token)
        .bind(&authorization.create_time)
        .bind(&authorization.access_token_id)
        .bind(&authorization.access_token_exp)
        .bind(&authorization.access_token_iat)
        .bind(&authorization.is_enabled)
        .fetch_one(db)
        .await;
    
    match r {
        Ok(v) => Ok(v),
        Err(e) => {
            error!(log, "{}", e);
            Err(error::err500())
        }
    }
}


// 禁用授权
pub async fn disable_auth(id: i32, db: &sqlx::Pool<sqlx::Postgres>, log: &slog::Logger) -> Result<(), error::Error> {
    let r = sqlx::query("UPDATE authorizations SET is_enabled=0, update_time=$1 WHERE id=$2")
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
pub async fn get_by_id(id: i32, db: &sqlx::Pool<sqlx::Postgres>, log: &slog::Logger) -> Result<Option<Authorization>, error::Error> {
    let r = sqlx::query_as::<_, Authorization>(r#"
        SELECT a.*
        FROM authorizations a INNER JOIN users b ON a.user_id=b.id
        WHERE a.id=$1 AND b.is_enabled=1 AND b.is_del=0"#)
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
pub async fn get_by_uuid(uuid: uuid::Uuid, db: &sqlx::Pool<sqlx::Postgres>, log: &slog::Logger) -> Result<Option<Authorization>, error::Error> {
    let r = sqlx::query_as::<_, Authorization>("SELECT * FROM authorizations WHERE uuid=$1")
        .bind(uuid)
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
pub async fn update_auth(authorization: &Authorization, db: &sqlx::Pool<sqlx::Postgres>, log: &slog::Logger) -> Result<Authorization, error::Error> {
    let id = match authorization.id {
        Some(v) => v,
        None => 0,
    };

    if id <= 0 {
        error!(log, "update id error: {}", id);
        return Err(error::err500());
    }

    let mut sql1 = vec![format!("update_time = $1")];
    let mut sql_index = 2;

    if let Some(_) = authorization.refresh_token {
        sql1.push(format!("refresh_token = ${}", sql_index));
        sql_index += 1;
    }
    if let Some(_) = authorization.last_refresh_time {
        sql1.push(format!("last_refresh_time = ${}", sql_index));
        sql_index += 1;
    }
    if let Some(_) = authorization.access_token_id {
        sql1.push(format!("access_token_id = ${}", sql_index));
        sql_index += 1;
    }
    if let Some(_) = authorization.access_token_exp {
        sql1.push(format!("access_token_exp = ${}", sql_index));
        sql_index += 1;
    }
    if let Some(_) = authorization.access_token_iat {
        sql1.push(format!("access_token_iat = ${}", sql_index));
        sql_index += 1;
    }

    let sql = format!("UPDATE authorizations SET {} WHERE id = ${} RETURNING *", sql1.join(","), sql_index);

    let mut q = sqlx::query_as::<_, Authorization>(&sql);

    q = q.bind(Utc::now());
    if let Some(refresh_token) = authorization.refresh_token {
        q = q.bind(refresh_token);
    }
    if let Some(last_refresh_time) = authorization.last_refresh_time {
        q = q.bind(last_refresh_time);
    }
    if let Some(access_token_id) = authorization.access_token_id {
        q = q.bind(access_token_id);
    }
    if let Some(access_token_exp) = authorization.access_token_exp {
        q = q.bind(access_token_exp);
    }
    if let Some(access_token_iat) = authorization.access_token_iat {
        q = q.bind(access_token_iat);
    }
    q = q.bind(id);

    let r = q.fetch_one(db).await;
    
    match r {
        Ok(v) => Ok(v),
        Err(e) => {
            error!(log, "{}", e);
            Err(error::err500())
        }
    }
}