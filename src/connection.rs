use anyhow::{anyhow, Result};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use diesel::r2d2::ConnectionManager;
use lazy_static::lazy_static;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

embed_migrations!();

lazy_static! {
    static ref POOL: Pool = {
        let db_url = env::var("DATABASE_URL").expect("Database url not set");
        let manager = ConnectionManager::<PgConnection>::new(db_url);
        Pool::new(manager).expect("Failed to create db pool")
    };
}

pub fn init() {
    info!("Initializing DB");
    lazy_static::initialize(&POOL);
    let conn = connection().expect("Failed to get db connection");
    embedded_migrations::run(&conn).unwrap();
}

pub fn connection() -> Result<DbConnection> {
    POOL.get()
        .map_err(|e| anyhow!("Failed getting db connection: {}", e))
}

/// Establish a single connection to the database.
pub fn establish_connection() -> Result<PgConnection> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL");
    if let Ok(database_url) = database_url {
        PgConnection::establish(&database_url)
        .map_err(|e| anyhow!("Database error: {}", e))
    } else {
        Err(anyhow!("DATABASE_URL must be set"))
    }
}
