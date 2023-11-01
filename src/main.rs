use axum::{
    routing::{get, post},
    Router,
};
use migration::{Migrator, MigratorTrait};
use sea_orm::{prelude::*, Database};
use tracing::info;

mod handlers;

#[derive(Clone)]
pub(crate) struct AppState {
    db: DatabaseConnection,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    info!("Connecting to database!");

    let db: DatabaseConnection =
        Database::connect("postgres://postgres:postgres@127.0.0.1/database")
            .await
            .expect("There was an error while connecting to the database");

    Migrator::up(&db, None)
        .await
        .expect("There was an error while migrating the database");

    let state = AppState { db };

    let app = Router::new()
        .route("/api", get(handlers::index))
        .route("/api/config", get(handlers::config))
        .route("/api/login", get(handlers::login_get))
        .route("/api/login", post(handlers::login_post))
        .fallback(handlers::not_found)
        .with_state(state);

    info!("Listening on 0.0.0.0:3000!");

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
