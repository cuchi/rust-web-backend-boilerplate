use crate::todo::service::{self, ListOptions};
use crate::{context::ApiContext, error::ApiResult, todo::model::Todo};
use rocket::State;
use rocket_contrib::json::Json;

#[get("/todos?<is_done>")]
pub fn list(context: State<ApiContext>, is_done: Option<bool>) -> ApiResult<Json<Vec<Todo>>> {
    Ok(Json(service::list(
        &context,
        &ListOptions {
            is_done,
            name: None,
        },
    )?))
}
