use rocket::State;

use rocket_contrib::json::Json;

use diesel::prelude::*;
use crate::{
    schema::todos::dsl::*,
    todo::model::Todo,
    context::AppContext,
    error::AppResult,
};

#[get("/todos")]
pub fn list(context: State<AppContext>) -> AppResult<Json<Vec<Todo>>> {
    let db = context.get_db()?;
    let results = todos
        .filter(is_done.eq(true))
        .limit(5)
        .load::<Todo>(&db)
        .unwrap();
    Ok(Json(results))
}
