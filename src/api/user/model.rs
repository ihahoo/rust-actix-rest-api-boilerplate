use chrono::prelude::*;
use crate::lib::error;
use super::{User, UserInfo};

pub async fn get_by_id(id: i32, db: &sqlx::Pool<sqlx::Postgres>, log: &slog::Logger) -> Result<Option<User>, error::Error> {
    let r = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id=$1")
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

pub async fn get_by_username(username: &str, db: &sqlx::Pool<sqlx::Postgres>, log: &slog::Logger) -> Result<Option<User>, error::Error> {
    let r = sqlx::query_as::<_, User>("SELECT * FROM users WHERE username=$1")
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

pub async fn insert(user: &User, db: &sqlx::Pool<sqlx::Postgres>, log: &slog::Logger) -> Result<User, error::Error> {
    let mut sql1 = vec![String::from("uuid")];
    let mut sql2 = vec![String::from("$1")];
    let mut sql_index = 2;

    if let Some(_) = &user.username {
        sql1.push(String::from("username"));
        sql2.push(format!("${}", sql_index));
        sql_index += 1;
    }

    if let Some(_) = &user.password {
        sql1.push(String::from("password"));
        sql2.push(format!("${}", sql_index));
        sql_index += 1;
    }

    if let Some(_) = &user.salt {
        sql1.push(String::from("salt"));
        sql2.push(format!("${}", sql_index));
        sql_index += 1;
    }

    if let Some(_) = &user.mobile {
        sql1.push(String::from("mobile"));
        sql2.push(format!("${}", sql_index));
        sql_index += 1;
    }

    sql1.push(String::from("create_time"));
    sql2.push(format!("${}", sql_index));
    sql_index += 1;

    if let Some(_) = &user.update_time {
        sql1.push(String::from("update_time"));
        sql2.push(format!("${}", sql_index));
        sql_index += 1;
    }

    sql1.push(String::from("is_del"));
    sql2.push(format!("${}", sql_index));
    sql_index += 1;

    sql1.push(String::from("is_enabled"));
    sql2.push(format!("${}", sql_index));
    sql_index += 1;

    sql1.push(String::from("user_type"));
    sql2.push(format!("${}", sql_index));

    let sql = format!("INSERT INTO users ({}) VALUES ({}) RETURNING *", sql1.join(","), sql2.join(","));
    
    let mut q = sqlx::query_as::<_, User>(&sql);

    if let Some(uuid) = &user.uuid {
        q = q.bind(uuid);
    } else {
        q = q.bind(uuid::Uuid::new_v4());
    }

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

    if let Some(create_time) = &user.create_time {
        q = q.bind(create_time);
    } else {
        q = q.bind(Utc::now());
    }

    if let Some(update_time) = &user.update_time {
        q = q.bind(update_time);
    }

    if let Some(is_enabled) = &user.is_enabled {
        q = q.bind(is_enabled);
    } else {
        q = q.bind(1);
    }

    if let Some(is_del) = &user.is_del {
        q = q.bind(is_del);
    } else {
        q = q.bind(0);
    }

    if let Some(user_type) = &user.user_type {
        q = q.bind(user_type);
    } else {
        q = q.bind(0);
    }

    let r = q.fetch_one(db).await;
    
    match r {
        Ok(v) => Ok(v),
        Err(e) => {
            error!(log, "{}", e);
            Err(error::err500())
        }
    }
}

pub async fn update(user: &User, db: &sqlx::Pool<sqlx::Postgres>, log: &slog::Logger) -> Result<User, error::Error> {
    let id = match user.id {
        Some(v) => v,
        None => 0,
    };

    if id <= 0 {
        error!(log, "update id error: {}", id);
        return Err(error::err500());
    }

    let mut sql1 = vec![format!("update_time = $1")];
    let mut sql_index = 2;

    if let Some(_) = &user.username {
        sql1.push(format!("username = ${}", sql_index));
        sql_index += 1;
    }
    if let Some(_) = &user.password {
        sql1.push(format!("password = ${}", sql_index));
        sql_index += 1;
    }
    if let Some(_) = &user.salt {
        sql1.push(format!("salt = ${}", sql_index));
        sql_index += 1;
    }
    if let Some(_) = &user.mobile {
        sql1.push(format!("mobile = ${}", sql_index));
        sql_index += 1;
    }
    if let Some(_) = &user.is_enabled {
        sql1.push(format!("is_enabled = ${}", sql_index));
        sql_index += 1;
    }
    if let Some(_) = &user.last_login_time {
        sql1.push(format!("last_login_time = ${}", sql_index));
        sql_index += 1;
    }
    if let Some(_) = &user.last_login_ip {
        sql1.push(format!("last_login_ip = ${}", sql_index));
        sql_index += 1;
    }
    if let Some(_) = &user.user_type {
        sql1.push(format!("user_type = ${}", sql_index));
        sql_index += 1;
    }

    let sql = format!("UPDATE users SET {} WHERE id = ${} RETURNING *", sql1.join(","), sql_index);

    let mut q = sqlx::query_as::<_, User>(&sql);

    q = q.bind(Utc::now());
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

    let r = q.fetch_one(db).await;
    
    match r {
        Ok(v) => Ok(v),
        Err(e) => {
            error!(log, "{}", e);
            Err(error::err500())
        }
    }
}

pub async fn update_last_login(login_time: DateTime<Utc>, ip: &str, user_id: i32, db: &sqlx::Pool<sqlx::Postgres>, log: &slog::Logger) -> Result<(), error::Error> {
    let r = sqlx::query(r#"UPDATE users SET last_login_time=$1, last_login_ip=$2 WHERE id=$3"#)
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

pub async fn delete(id: i32, db: &sqlx::Pool<sqlx::Postgres>, log: &slog::Logger) -> Result<(), error::Error> {
    let r = sqlx::query("UPDATE users SET is_del = 1 WHERE id=$1")
        .bind(id)
        .execute(db)
        .await;
    
    if let Err(err) = r {
        error!(log, "{}", err);
        return Err(error::err500());
    }

    Ok(())
}

pub async fn get_user_info_by_id(id: i32, db: &sqlx::Pool<sqlx::Postgres>, log: &slog::Logger) -> Result<Option<UserInfo>, error::Error> {
    let r = sqlx::query_as::<_, UserInfo>(r#"
        SELECT id, username, uuid, mobile, last_login_time, last_login_ip, user_type FROM users
        WHERE id = $1 AND is_del=0 AND is_enabled=1"#)
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