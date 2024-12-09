use std::{env, sync::LazyLock};

use anyhow::Context;
use diesel::{r2d2::ConnectionManager, MysqlConnection};
use dotenvy::dotenv;

pub mod user;

pub mod schema;

type DbPool = diesel::r2d2::Pool<ConnectionManager<MysqlConnection>>;
pub type DbConn = diesel::r2d2::PooledConnection<ConnectionManager<MysqlConnection>>;

// diesel::r2d2::Pool 자체적으로 동시성 처리함 
pub static DB_POOL : LazyLock<DbPool> = LazyLock::new(|| {
    dotenv().ok();
    let url = get_database_url();
    let manager = ConnectionManager::<MysqlConnection>::new(url);
    diesel::r2d2::Pool::builder()
        .max_size(15)
        .build(manager)
        .expect("Failed to create pool")
});

fn get_database_url() -> String {
    dotenv().ok();
    if cfg!(test) {
        env::var("DATABASE_URL_TEST").expect("DATABASE_URL_TEST is not in env file")
    } else {
        env::var("DATABASE_URL").expect("DATABASE_URL is not in env file")
    }
}

pub fn get_connection() -> anyhow::Result<DbConn> {
    let pool = &*DB_POOL;
    pool.get().context("Failed to get connection")
}
