use ::entity::user;
use axum::{extract::State, Json};
use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};
use std::env;

use crate::AppState;

pub(crate) async fn not_found() -> &'static str {
    "Not found"
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

#[derive(Debug, Serialize)]
pub(crate) struct LoginGetResponse {
    user: Option<user::Model>,
}

/// In case you forget your password
pub(crate) async fn login_get(
    state: State<AppState>,
    login: Json<LoginGetData>,
) -> Json<LoginGetResponse> {
    let user = user::Entity::find()
        .filter(user::Column::Email.eq(login.email.clone()))
        .one(&state.db)
        .await
        .expect("There was an error");

    Json(LoginGetResponse { user })
}

#[derive(Deserialize)]
pub(crate) struct LoginData {
    email: String,
    password: String,
}

/// Sign in into the admin page
pub(crate) async fn login_post(state: State<AppState>, login: Json<LoginData>) -> String {
    let user = user::Entity::find()
        .filter(user::Column::Email.eq(login.email.clone()))
        // TODO: a friend told me hash_function is safer
        .filter(user::Column::Password.eq(login.password.clone()))
        .one(&state.db)
        .await
        .expect("There was an error");

    if user.is_some() {
        env::var("HAXAGON_FLAG").unwrap_or("haxagon{this_is_the_flag}".into())
    } else {
        "Unauthorized".into()
    }
}
