use diesel::r2d2;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::PgConnection;
use dotenv::dotenv;
use std::env;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub fn get_connection_pool() -> Pool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    return r2d2::Pool::new(manager).unwrap();
}
