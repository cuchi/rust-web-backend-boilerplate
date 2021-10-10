use crate::db::{self, DbConnection};
use crate::error::ApiResult;

pub struct ApiContext {
    pub pool: db::Pool,
}

impl ApiContext {
    pub fn get_db(&self) -> ApiResult<DbConnection> {
        Ok(self.pool.get()?)
    }
}
