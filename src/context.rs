use crate::db::{self, DbConnection};
use crate::error::AppResult;

pub struct AppContext {
    pub pool: db::Pool,
}

impl AppContext {
    pub fn get_db(&self) -> AppResult<DbConnection> {
        Ok(self.pool.get()?)
    }
}
