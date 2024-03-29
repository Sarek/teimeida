use crate::cleanup::cleanup;
use axum::{
    extract::DefaultBodyLimit,
    routing::{get, get_service},
    Router,
};
use axum_htpasswd::{Encoding, FileAuth};
use simple_logger::SimpleLogger;
use tokio::{fs::File, spawn};
use tokio_schedule::{every, Job};
use tower_http::services::{ServeDir, ServeFile};
use tower_http::validate_request::ValidateRequestHeaderLayer;

#[macro_use]
extern crate log;

mod cleanup;
mod overview;
mod retrieve;
mod share;

#[tokio::main]
async fn main() {
    SimpleLogger::new()
        .with_colors(true)
        .with_level(log::LevelFilter::Debug)
        .env()
        .with_utc_timestamps()
        .init()
        .unwrap();

    let cleaner = every(1).day().at(0, 0, 0).perform(cleanup);
    spawn(cleaner);

    let index = ServeFile::new("assets/index.html");
    let assets = ServeDir::new("assets");
    let new_static = ServeFile::new("templates/new.html");

    let app = Router::new()
        .route_service("/", index)
        .route("/new", get_service(new_static).post(share::share_handler))
        .route("/overview", get(overview::overview_handler))
        .layer(DefaultBodyLimit::max(1024 * 1024 * 1024))
        .route_layer(ValidateRequestHeaderLayer::custom(
            FileAuth::new(&mut File::open("config/users.conf").await.unwrap(), Encoding::PlainText).await,
        ))
        .route("/retrieve/:id", get(retrieve::retrieve_handler))
        .route_service("/*path", assets);

    info!("Teimeida starting to serve on port 8080");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
