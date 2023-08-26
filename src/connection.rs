use anyhow::{anyhow, Result};
use diesel::{pg::PgConnection, r2d2::ConnectionManager, Connection};
use dotenv::dotenv;
use lazy_static::lazy_static;
use std::env;

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

pub fn init() -> Result<()> {
    info!("Initializing DB");
    lazy_static::initialize(&POOL);
    let conn = connection().map_err(|e| anyhow!("Failed to get db connection {e}"))?;
    embedded_migrations::run(&conn).map_err(|e| anyhow!("Failed to run migrations {e}"))
}

pub fn connection() -> Result<DbConnection> {
    POOL.get()
        .map_err(|e| anyhow!("Failed getting db connection: {e}"))
}

/// Establish a single connection to the database.
pub fn establish_connection() -> Result<PgConnection> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").map_err(|_| anyhow!("DATABASE_URL must be set"))?;
    PgConnection::establish(&database_url).map_err(|e| anyhow!("Database error: {e}"))
}

#[cfg(feature = "enableactix")]
pub fn establish_connection_or_http_error() -> Result<PgConnection, error_utils::msghttp::MsgHttp> {
    establish_connection().map_err(|_| error_utils::msghttp::MsgHttp {
        msg: "Database error".to_string(),
        status: 500,
    })
}
