use crate::cleanup::cleanup;
use axum::{
    extract::DefaultBodyLimit,
    routing::{get, get_service},
    Router, Server,
};
use simple_logger::SimpleLogger;
use tokio::{fs::File, spawn};
use tokio_schedule::{every, Job};
use tower_http::services::ServeFile;
use tower_http::validate_request::ValidateRequestHeaderLayer;

#[macro_use]
extern crate log;

mod cleanup;
mod fileauth;
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
    let new_static = ServeFile::new("assets/new.html");

    let app = Router::new()
        .route_service("/", index)
        .route("/new", get_service(new_static).post(share::share_handler))
        .layer(DefaultBodyLimit::max(1024 * 1024 * 1024))
        .route_layer(ValidateRequestHeaderLayer::custom(
            fileauth::FileAuth::new(&mut File::open("config/users.conf").await.unwrap()).await,
        ))
        //.route_layer(ValidateRequestHeaderLayer::basic("user", "secret"))
        .route("/retrieve/:id", get(retrieve::retrieve_handler));

    info!("Teimeida starting to serve on port 8080");
    Server::bind(&([0, 0, 0, 0], 8080).into())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
