use mobc::Pool;
use mobc::async_trait;
use mobc::Manager;
use redis::aio::Connection;
use redis::Client;
use std::time::Duration;

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