use poem_openapi::Object;
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Object, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub uuid: Uuid,
    pub username: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String
}