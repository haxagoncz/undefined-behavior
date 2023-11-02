use ::entity::user;
use axum::{extract::State, Json};
use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};
use std::env;

use crate::{errors::CustomError, AppState};

pub(crate) async fn not_found() -> CustomError {
    CustomError::NotFound
}

pub(crate) async fn index() -> &'static str {
    r#"Endpoints:
- /api/login
- /api/config"#
}

#[derive(Serialize)]
pub(crate) struct AppInfo {
    repository: String,
    sha: String,
    version: String,
}

pub(crate) async fn config() -> Json<AppInfo> {
    Json(AppInfo {
        repository: String::from("https://github.com/haxagoncz/the_server_shouldnt_do_that"),
        sha: String::from(env!("GIT_HASH")),
        version: String::from("1.0.0"),
    })
}

#[derive(Deserialize)]
pub(crate) struct LoginGetData {
    email: String,
}

/// In case you forget your password
pub(crate) async fn login_get(
    state: State<AppState>,
    login: Json<LoginGetData>,
) -> Result<Json<user::Model>, CustomError> {
    let user = user::Entity::find()
        .filter(user::Column::Email.eq(login.email.clone()))
        .one(&state.db)
        .await?;

    if let Some(user) = user {
        Ok(Json(user))
    } else {
        Err(CustomError::UserNotFound)
    }
}

#[derive(Deserialize)]
pub(crate) struct LoginData {
    email: String,
    password: String,
}

/// Sign in into the admin page
pub(crate) async fn login_post(
    state: State<AppState>,
    login: Json<LoginData>,
) -> Result<String, CustomError> {
    let user = user::Entity::find()
        .filter(user::Column::Email.eq(login.email.clone()))
        // TODO: a friend told me hash_function is safer
        .filter(user::Column::Password.eq(login.password.clone()))
        .one(&state.db)
        .await?;

    if user.is_some() {
        Ok(env::var("HAXAGON_FLAG").unwrap_or("haxagon{this_is_the_flag}".into()))
    } else {
        Err(CustomError::Unauthorized)
    }
}
