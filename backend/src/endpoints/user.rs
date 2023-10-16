use std::sync::Arc;
use poem_openapi::{ApiResponse, OpenApi, Tags};
use poem_openapi::param::Path;
use poem_openapi::payload::{Json, PlainText};
use uuid::Uuid;
use crate::db::Db;
use crate::structures::User;

#[derive(ApiResponse)]
enum GetUserResponse {
    #[oai(status=200)]
    Ok(Json<User>),

    #[oai(status = 400)]
    NotFound(PlainText<String>)
}

#[derive(Tags)]
enum ApiTags {
    User,
}

pub struct UserApi {
    pub pool: Arc<Db>
}

#[OpenApi]
impl UserApi {
    #[oai(path = "/user/:id", method = "get", tag = "ApiTags::User")]
    async fn get(&self, id: Path<i32>) -> PlainText<String> {
        PlainText("Hello world!".parse().unwrap())
    }
}