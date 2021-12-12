use sqlx::postgres::PgPoolOptions;
use std::time::Duration;

pub async fn conn(settings: &config::Config) -> sqlx::Pool<sqlx::Postgres> {
    let user = settings.get::<String>("pg.user").unwrap();
    let password = settings.get::<String>("pg.password").unwrap();
    let host = settings.get::<String>("pg.host").unwrap();
    let port = settings.get::<String>("pg.port").unwrap();
    let dbname = settings.get::<String>("pg.dbname").unwrap();
    let connect_timeout = settings.get::<u64>("pg.connect_timeout").unwrap();
    let idle_timeout = settings.get::<u64>("pg.idle_timeout").unwrap();
    let max = settings.get::<u32>("pg.max").unwrap();

    let pool = PgPoolOptions::new()
        .max_connections(max)
        .idle_timeout(Duration::new(idle_timeout, 0))
        .connect_timeout(Duration::new(connect_timeout, 0))
        .connect(&format!("postgres://{}:{}@{}:{}/{}", user, password, host, port, dbname)[..])
        .await
        .unwrap();
    
    pool
}