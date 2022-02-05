use axum::{
    routing::{get},
    AddExtensionLayer, Router,
};
use dotenv::{dotenv, var};
use sqlx::{
    migrate::Migrator,
    sqlite::{SqliteConnectOptions, SqlitePool, SqlitePoolOptions},
};
use std::path::Path;
use std::str::FromStr;
use tower_http::trace::TraceLayer;
use tower::ServiceBuilder;
use bollard::Docker;

mod routes;
use routes::{status};

#[tokio::main]
async fn main() {
    // Load env variables
    dotenv().ok();
    let host = var("HOST").unwrap_or_else(|_| "0.0.0.0:3000".to_string());

    // Setup Logging
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "pen=debug,tower_http=debug")
    }
    tracing_subscriber::fmt::init();

    // Connect to Docker
    let dock = Docker::connect_with_socket_defaults().expect("Could not connect to docker");

    // Setup DB connection & run migrations
    let uri = var("DATABASE_URL").unwrap_or_else(|_| "sqlite:reddock.db".to_string());
    let con = SqliteConnectOptions::from_str(&uri)
        .expect("Error creating sqlite connection")
        .create_if_missing(true);
    let db = SqlitePoolOptions::new()
        .connect_with(con)
        .await
        .expect("Error creating Sqlite Pool");
    let migrate = Migrator::new(Path::new("./migrations"))
        .await
        .expect("Could not find migrations directory");
    migrate.run(&db).await.expect("Error running migrations");

    // Build router
    let app = app(db, dock);

    // Start the server
    tracing::debug!("listening on {}", host);
    axum::Server::bind(
        &host
            .parse()
            .expect("HOST in .env not valid <ip>:<port> format"),
    )
    .serve(app.into_make_service())
    .await
    .unwrap();
}

fn app(db: SqlitePool, dock: Docker) -> Router {
    Router::new()
        // Add middleware for every route
        .route("/status", get(status::handler))
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(AddExtensionLayer::new(db))
                .layer(AddExtensionLayer::new(dock))
                .into_inner(),
        )
}
