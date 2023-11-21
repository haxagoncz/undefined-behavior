use ::entity::user;
use axum::{extract::State, http::HeaderMap};
use axum_media::{AnyMedia, AnyMediaIntoResponse};
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

pub(crate) async fn config(headers: HeaderMap) -> AnyMediaIntoResponse<AppInfo> {
    AnyMedia(AppInfo {
        repository: String::from("https://github.com/haxagoncz/undefined-behavior"),
        sha: String::from(env!("GIT_HASH")),
        version: String::from(env!("CARGO_PKG_VERSION")),
    })
    .with_mime_str(
        headers
            .get("accept")
            .map(|s| s.to_str().unwrap_or(""))
            .unwrap_or(""),
    )
}

#[derive(Deserialize)]
pub(crate) struct LoginGetData {
    email: String,
}

/// In case you forget your password
pub(crate) async fn login_get(
    headers: HeaderMap,
    state: State<AppState>,
    AnyMedia(login): AnyMedia<LoginGetData>,
) -> Result<AnyMediaIntoResponse<user::Model>, CustomError> {
    let user = user::Entity::find()
        .filter(user::Column::Email.eq(login.email.clone()))
        .one(&state.db)
        .await?;

    if let Some(user) = user {
        Ok(AnyMedia(user).with_mime_str(
            headers
                .get("accept")
                .map(|s| s.to_str().unwrap_or(""))
                .unwrap_or(""),
        ))
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
    AnyMedia(login): AnyMedia<LoginData>,
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
