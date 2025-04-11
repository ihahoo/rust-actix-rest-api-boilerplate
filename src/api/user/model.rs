use chrono::prelude::*;
use crate::lib::error;
use super::{User, UserInfo};

pub async fn get_by_id(id: i32, db: &sqlx::Pool<sqlx::MySql>, log: &slog::Logger) -> Result<Option<User>, error::Error> {
    let r = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id=?")
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

pub async fn get_by_username(username: &str, db: &sqlx::Pool<sqlx::MySql>, log: &slog::Logger) -> Result<Option<User>, error::Error> {
    let r = sqlx::query_as::<_, User>("SELECT * FROM users WHERE username=?")
        .bind(username)
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

pub async fn insert(user: &User, db: &sqlx::Pool<sqlx::MySql>, log: &slog::Logger) -> Result<User, error::Error> {
    // 准备插入数据
    let uuid = match &user.uuid {
        Some(uuid) => uuid.clone(),
        None => uuid::Uuid::new_v4().to_string(),
    };
    
    let create_time = match &user.create_time {
        Some(time) => *time,
        None => Utc::now(),
    };
    
    let is_enabled = match &user.is_enabled {
        Some(val) => *val,
        None => 1,
    };
    
    let is_del = match &user.is_del {
        Some(val) => *val,
        None => 0,
    };
    
    let user_type = match &user.user_type {
        Some(val) => *val,
        None => 0,
    };
    
    // 执行插入操作
    let r = sqlx::query(r#"
        INSERT INTO users 
        (uuid, username, password, salt, mobile, create_time, update_time, is_del, is_enabled, user_type)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#)
        .bind(&uuid)
        .bind(&user.username)
        .bind(&user.password)
        .bind(&user.salt)
        .bind(&user.mobile)
        .bind(create_time)
        .bind(&user.update_time)
        .bind(is_del)
        .bind(is_enabled)
        .bind(user_type)
        .execute(db)
        .await;
    
    if let Err(e) = r {
        error!(log, "{}", e);
        return Err(error::err500());
    }
    
    // 获取最后插入的ID
    let last_id = sqlx::query_scalar::<_, i64>("SELECT LAST_INSERT_ID()")
        .fetch_one(db)
        .await;
    
    if let Err(e) = last_id {
        error!(log, "{}", e);
        return Err(error::err500());
    }
    
    // 查询刚插入的记录
    let r = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
        .bind(last_id.unwrap())
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

pub async fn update(user: &User, db: &sqlx::Pool<sqlx::MySql>, log: &slog::Logger) -> Result<(), error::Error> {
    let id = match user.id {
        Some(v) => v,
        None => 0,
    };

    if id <= 0 {
        error!(log, "update id error: {}", id);
        return Err(error::err500());
    }

    let mut query = String::from("UPDATE users SET ");
    let mut params: Vec<String> = Vec::new();

    // 固定更新字段
    params.push("update_time = ?".to_string());

    // 可选更新字段
    if user.username.is_some() {
        params.push("username = ?".to_string());
    }
    if user.password.is_some() {
        params.push("password = ?".to_string());
    }
    if user.salt.is_some() {
        params.push("salt = ?".to_string());
    }
    if user.mobile.is_some() {
        params.push("mobile = ?".to_string());
    }
    if user.is_enabled.is_some() {
        params.push("is_enabled = ?".to_string());
    }
    if user.last_login_time.is_some() {
        params.push("last_login_time = ?".to_string());
    }
    if user.last_login_ip.is_some() {
        params.push("last_login_ip = ?".to_string());
    }
    if user.user_type.is_some() {
        params.push("user_type = ?".to_string());
    }

    if params.is_empty() {
        return Ok(());
    }

    query.push_str(&params.join(", "));
    query.push_str(" WHERE id = ?");

    // 按顺序绑定参数
    let mut q = sqlx::query(&query)
        .bind(Utc::now());  // 绑定 update_time

    if let Some(username) = &user.username {
        q = q.bind(username);
    }
    if let Some(password) = &user.password {
        q = q.bind(password);
    }
    if let Some(salt) = &user.salt {
        q = q.bind(salt);
    }
    if let Some(mobile) = &user.mobile {
        q = q.bind(mobile);
    }
    if let Some(is_enabled) = &user.is_enabled {
        q = q.bind(is_enabled);
    }
    if let Some(last_login_time) = &user.last_login_time {
        q = q.bind(last_login_time);
    }
    if let Some(last_login_ip) = &user.last_login_ip {
        q = q.bind(last_login_ip);
    }
    if let Some(user_type) = &user.user_type {
        q = q.bind(user_type);
    }
    q = q.bind(id);

    let r = q.execute(db).await;
    
    if let Err(e) = r {
        error!(log, "{}", e);
        return Err(error::err500());
    }
    
    Ok(())
}

pub async fn update_last_login(login_time: DateTime<Utc>, ip: &str, user_id: i32, db: &sqlx::Pool<sqlx::MySql>, log: &slog::Logger) -> Result<(), error::Error> {
    let r = sqlx::query(r#"UPDATE users SET last_login_time=?, last_login_ip=? WHERE id=?"#)
        .bind(login_time)
        .bind(ip)
        .bind(user_id)
        .execute(db)
        .await;
    
    if let Err(err) = r {
        error!(log, "{}", err);
        return Err(error::err500());
    }

    Ok(())
}

pub async fn delete(id: i32, db: &sqlx::Pool<sqlx::MySql>, log: &slog::Logger) -> Result<(), error::Error> {
    let r = sqlx::query("UPDATE users SET is_del = 1 WHERE id=?")
        .bind(id)
        .execute(db)
        .await;
    
    if let Err(err) = r {
        error!(log, "{}", err);
        return Err(error::err500());
    }

    Ok(())
}

pub async fn get_user_info_by_id(id: i32, db: &sqlx::Pool<sqlx::MySql>, log: &slog::Logger) -> Result<Option<UserInfo>, error::Error> {
    let r = sqlx::query_as::<_, UserInfo>(r#"
        SELECT id, username, uuid, mobile, last_login_time, last_login_ip, user_type FROM users
        WHERE id = ? AND is_del=0 AND is_enabled=1"#)
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