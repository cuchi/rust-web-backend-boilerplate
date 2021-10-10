use crate::context::ApiContext;
use crate::error::ApiResult;
use crate::schema::todos::dsl;
use crate::todo::model::Todo;
use diesel::prelude::*;

pub struct ListOptions {
    pub is_done: Option<bool>,
    pub name: Option<String>,
}

pub fn list(context: &ApiContext, options: &ListOptions) -> ApiResult<Vec<Todo>> {
    let db = context.get_db()?;
    let mut query = dsl::todos.into_boxed();

    let ListOptions { is_done, .. } = options;
    if let Some(is_done) = is_done {
        query = query.filter(dsl::is_done.eq(is_done));
    }

    Ok(query.load::<Todo>(&db)?)
}
