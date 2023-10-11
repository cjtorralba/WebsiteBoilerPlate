use std::collections::HashMap;
use axum::extract::{Query, State};
use axum::headers::Header;
use axum::response::Html;
use crate::get_access_token;

/// Root routing page. just the "/" route.
/// # Returns:
/// [Result](Result)<[Html](Html)<[String](String)>, [AppError](AppError)>
///
/// # Arguments:
/// Uses dependency injection to get:
/// * [State](State)
/// * [OptionalClaims](OptionalClaims)
pub async fn root(
) -> String {

    "hello world".to_string()
}