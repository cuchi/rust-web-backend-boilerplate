use uuid::Uuid;
use serde::Serialize;

#[derive(Debug, Clone, Queryable, Serialize)]
pub struct Todo {
    pub id: Uuid,
    pub name: String,
    pub is_done: bool,
}
