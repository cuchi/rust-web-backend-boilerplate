use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Clone, Queryable, Serialize)]
pub struct Todo {
    pub id: Uuid,
    pub name: String,
    pub is_done: bool,
}
