use std::sync::Arc;
use poem_openapi::{ApiResponse, OpenApi, Tags};
use poem_openapi::param::Path;
use poem_openapi::payload::{Json, PlainText};
use poem::error::InternalServerError;
use poem::Result;
use uuid::Uuid;
use crate::db::Db;
use crate::structures::Account;


#[derive(ApiResponse)]
enum GetAccountResponse {
    #[oai(status = 200)]
    Ok(Json<Account>),

    #[oai(status = 400)]
    NotFound(PlainText<String>),
}

#[derive(Tags)]
enum ApiTags {
    Account,
}

pub struct AccountApi {
    pub pool: Arc<Db>
}

#[OpenApi]
impl AccountApi {
    #[oai(path = "/account", method = "get", tag = "ApiTags::Account")]
    async fn get(&self, id: Path<Uuid>) -> PlainText<String> {
        PlainText("Hello world!".parse().unwrap())
    }
}