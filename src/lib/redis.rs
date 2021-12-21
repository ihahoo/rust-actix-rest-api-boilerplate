use mobc::Pool;
use mobc::async_trait;
use mobc::Manager;
use redis::aio::Connection;
use redis::Client;
use std::time::Duration;
use super::error;

pub struct RedisConnectionManager {
    client: Client,
}

impl RedisConnectionManager {
    pub fn new(c: Client) -> Self {
        Self { client: c }
    }
}

#[async_trait]
impl Manager for RedisConnectionManager {
    type Connection = Connection;
    type Error = redis::RedisError;

    async fn connect(&self) -> Result<Self::Connection, Self::Error> {
        let c = self.client.get_async_connection().await?;
        Ok(c)
    }

    async fn check(&self, mut conn: Self::Connection) -> Result<Self::Connection, Self::Error> {
        redis::cmd("PING").query_async(&mut conn).await?;
        Ok(conn)
    }
}

pub async fn conn(settings: &config::Config) -> Pool<RedisConnectionManager> {
    let host = settings.get::<String>("redis.host").unwrap();
    let port = settings.get::<String>("redis.port").unwrap();
    let password = settings.get::<String>("redis.password").unwrap();
    let db = settings.get::<String>("redis.db").unwrap();
    let pool_get_timeout_seconds = settings.get::<u64>("redis.pool_get_timeout_seconds").unwrap();
    let pool_max_open = settings.get::<u64>("redis.pool_max_open").unwrap();
    let pool_max_idle = settings.get::<u64>("redis.pool_max_idle").unwrap();
    let pool_max_lifetime_seconds = settings.get::<u64>("redis.pool_max_lifetime_seconds").unwrap();

    let client = redis::Client::open(&format!("redis://:{}@{}:{}/{}", password, host, port, db)[..]).unwrap();
    let manager = RedisConnectionManager::new(client);
    let pool = Pool::builder()
        .get_timeout(Some(Duration::from_secs(pool_get_timeout_seconds)))
        .max_open(pool_max_open)
        .max_idle(pool_max_idle)
        .max_lifetime(Some(Duration::from_secs(pool_max_lifetime_seconds)))
        .build(manager);
    
    pool
}

// 设置过期时间（秒）
pub async fn expire(key: String, value: i64, pool: &mobc::Pool<RedisConnectionManager>, log: &slog::Logger)  -> Result<(), error::Error> {
    let mut con = pool.get().await.unwrap();
    let result = redis::cmd("EXPIRE").arg(key).arg(value).query_async::<_, i16>(&mut con as &mut redis::aio::Connection).await;
    if let Err(err) = result {
        error!(log, "{}", err);
        return Err(error::err500());
    }

    Ok(())
}

// 获取某key的过期时间(秒)
pub async fn get_expire(key: String, pool: &mobc::Pool<RedisConnectionManager>, log: &slog::Logger)  -> Result<i64, error::Error> {
    let mut con = pool.get().await.unwrap();
    let result = redis::cmd("TTL").arg(key).query_async::<_, i64>(&mut con as &mut redis::aio::Connection).await;
    match result {
        Ok(v) => Ok(v),
        Err(e) => {
            error!(log, "{}", e);
            Err(error::err500())
        }
    }
}

pub async fn del(key: String, pool: &mobc::Pool<RedisConnectionManager>, log: &slog::Logger)  -> Result<(), error::Error> {
    let mut con = pool.get().await.unwrap();
    let result = redis::cmd("DEL").arg(key).query_async::<_, i32>(&mut con as &mut redis::aio::Connection).await;
    if let Err(err) = result {
        error!(log, "{}", err);
        return Err(error::err500());
    }

    Ok(())
}

pub async fn has_key(key: String, pool: &mobc::Pool<RedisConnectionManager>, log: &slog::Logger)  -> Result<bool, error::Error> {
    let mut con = pool.get().await.unwrap();
    let result = redis::cmd("EXISTS").arg(key).query_async::<_, i32>(&mut con as &mut redis::aio::Connection).await;
    match result {
        Ok(v) => {
            if v > 0 {
                Ok(true)
            } else {
                Ok(false)
            }
        },
        Err(e) => {
            error!(log, "{}", e);
            Err(error::err500())
        }
    }
}

pub async fn set<T: redis::ToRedisArgs>(key: String, value: T, pool: &mobc::Pool<RedisConnectionManager>, log: &slog::Logger)  -> Result<(), error::Error> {
    let mut con = pool.get().await.unwrap();
    let result = redis::cmd("SET").arg(key).arg(value).query_async::<_, String>(&mut con as &mut redis::aio::Connection).await;
    if let Err(err) = result {
        error!(log, "{}", err);
        return Err(error::err500());
    }

    Ok(())
}

pub async fn set_with_expire<T: redis::ToRedisArgs>(key: String, value: T, time: i64, pool: &mobc::Pool<RedisConnectionManager>, log: &slog::Logger)  -> Result<(), error::Error> {
    let mut con = pool.get().await.unwrap();
    let result = redis::cmd("SET").arg(key).arg(value).arg("EX").arg(time).query_async::<_, String>(&mut con as &mut redis::aio::Connection).await;
    if let Err(err) = result {
        error!(log, "{}", err);
        return Err(error::err500());
    }

    Ok(())
}

pub async fn get<T: redis::FromRedisValue>(key: String, pool: &mobc::Pool<RedisConnectionManager>, log: &slog::Logger)  -> Result<T, error::Error> {
    let mut con = pool.get().await.unwrap();
    let result = redis::cmd("GET").arg(key).query_async::<_, T>(&mut con as &mut redis::aio::Connection).await;
    match result {
        Ok(v) => Ok(v),
        Err(e) => {
            error!(log, "{}", e);
            Err(error::err500())
        }
    }
}



pub async fn hset<T: redis::ToRedisArgs>(key: String, item: String, value: T, pool: &mobc::Pool<RedisConnectionManager>, log: &slog::Logger)  -> Result<(), error::Error> {
    let mut con = pool.get().await.unwrap();
    let result = redis::cmd("HSET").arg(key).arg(item).arg(value).query_async::<_, i32>(&mut con as &mut redis::aio::Connection).await;
    if let Err(err) = result {
        error!(log, "{}", err);
        return Err(error::err500());
    }

    Ok(())
}

pub async fn hset_with_expire<T: redis::ToRedisArgs>(key: String, item: String, value: T, time: i64, pool: &mobc::Pool<RedisConnectionManager>, log: &slog::Logger)  -> Result<(), error::Error> {
    let mut con = pool.get().await.unwrap();
    let result = redis::cmd("HSET").arg(&key).arg(item).arg(value).query_async::<_, i32>(&mut con as &mut redis::aio::Connection).await;
    if let Err(err) = result {
        error!(log, "{}", err);
        return Err(error::err500());
    }

    if time > 0 {
        expire(key, time, &pool, &log).await?;
    }

    Ok(())
}

pub async fn hget<T: redis::FromRedisValue>(key: String, item: String, pool: &mobc::Pool<RedisConnectionManager>, log: &slog::Logger)  -> Result<T, error::Error> {
    let mut con = pool.get().await.unwrap();
    let result = redis::cmd("HGET").arg(key).arg(item).query_async::<_, T>(&mut con as &mut redis::aio::Connection).await;
    match result {
        Ok(v) => Ok(v),
        Err(e) => {
            error!(log, "{}", e);
            Err(error::err500())
        }
    }
}

pub async fn hhas_key(key: String, item: String, pool: &mobc::Pool<RedisConnectionManager>, log: &slog::Logger)  -> Result<bool, error::Error> {
    let mut con = pool.get().await.unwrap();
    let result = redis::cmd("HEXISTS").arg(key).arg(item).query_async::<_, i32>(&mut con as &mut redis::aio::Connection).await;
    match result {
        Ok(v) => {
            if v > 0 {
                Ok(true)
            } else {
                Ok(false)
            }
        },
        Err(e) => {
            error!(log, "{}", e);
            Err(error::err500())
        }
    }
}

pub async fn hdel(key: String, item: String, pool: &mobc::Pool<RedisConnectionManager>, log: &slog::Logger)  -> Result<(), error::Error> {
    let mut con = pool.get().await.unwrap();
    let result = redis::cmd("HDEL").arg(key).arg(item).query_async::<_, i32>(&mut con as &mut redis::aio::Connection).await;
    if let Err(err) = result {
        error!(log, "{}", err);
        return Err(error::err500());
    }

    Ok(())
}