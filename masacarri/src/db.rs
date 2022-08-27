use std::env;

use actix_session::storage::RedisSessionStore;
use diesel::{r2d2::ConnectionManager, PgConnection, Connection};
use r2d2::PooledConnection;

pub type MainDbConnection = PgConnection;
pub type DbManager = ConnectionManager<MainDbConnection>;
pub type MainDbPooledConnection = PooledConnection<DbManager>;
pub type Pool = r2d2::Pool<DbManager>;
type SessionStore = RedisSessionStore;

pub fn establish_main_db() -> MainDbConnection {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    MainDbConnection::establish(&database_url)
        .expect("Failed to establish connection to postgreSQL")
}

pub fn establish_main_db_pool() -> Pool {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<MainDbConnection>::new(&database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to establish connection to postgreSQL")
}

pub async fn establish_session_db() -> SessionStore {
    let session_database_url = env::var("SESSION_DATABASE_URL")
        .expect("SESSION_DATABASE_URL must be set");
    RedisSessionStore::new(session_database_url)
        .await
        .expect("Failed to establish connection to redis")
}
