use sqlx::mysql::MySqlPoolOptions;
use std::time::Duration;

pub async fn conn(settings: &config::Config) -> sqlx::Pool<sqlx::MySql> {
    let user = settings.get::<String>("mysql.user").unwrap();
    let password = settings.get::<String>("mysql.password").unwrap();
    let host = settings.get::<String>("mysql.host").unwrap();
    let port = settings.get::<String>("mysql.port").unwrap();
    let dbname = settings.get::<String>("mysql.dbname").unwrap();
    let connect_timeout = settings.get::<u64>("mysql.connect_timeout").unwrap();
    let idle_timeout = settings.get::<u64>("mysql.idle_timeout").unwrap();
    let max = settings.get::<u32>("mysql.max").unwrap();

    let pool = MySqlPoolOptions::new()
        .max_connections(max)
        .idle_timeout(Duration::new(idle_timeout, 0))
        .acquire_timeout(Duration::new(connect_timeout, 0))
        .connect(&format!("mysql://{}:{}@{}:{}/{}", user, password, host, port, dbname)[..])
        .await
        .unwrap();
    
    pool
}