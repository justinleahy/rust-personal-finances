use crate::model::Db;
use std::path::Path;
use std::sync::Arc;
use warp::Filter;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Web server failed to start because web-folder '{0}' not found.")]
    FailStartWebFolderNotFound(String)
}