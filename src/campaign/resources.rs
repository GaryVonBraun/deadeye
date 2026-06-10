use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct Campaign {
    pub id: Uuid,
    pub name: String,
    pub money: i32,
}
