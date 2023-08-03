use crate::cleanup::cleanup;
use axum::{
    extract::DefaultBodyLimit,
    routing::{get, get_service},
    Router, Server,
};
use simple_logger::SimpleLogger;
use tokio::spawn;
use tokio_schedule::{every, Job};
use tower_http::services::ServeFile;

#[macro_use]
extern crate log;

mod cleanup;
mod retrieve;
mod share;

#[tokio::main]
async fn main() {
    SimpleLogger::new().init().unwrap();

    let cleaner = every(1).day().at(0, 0, 0).perform(cleanup);
    spawn(cleaner);

    let index = ServeFile::new("assets/index.html");
    let new_static = ServeFile::new("assets/new.html");

    let app = Router::new()
        .route("/new", get_service(new_static).post(share::share_handler))
        .route("/retrieve/:id", get(retrieve::retrieve_handler))
        .route_service("/", index)
        .layer(DefaultBodyLimit::max(1024 * 1024 * 1024));

    info!("Teimeida starting to serve on port 8080");
    Server::bind(&([0, 0, 0, 0], 8080).into())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
