mod data;

use axum::{
    extract::Multipart,
    http::StatusCode,
    response::{Html, IntoResponse},
};
use chrono::NaiveDate;
use tokio::fs::read_to_string;

use self::data::ShareData;

pub async fn share_handler(mut multipart: Multipart) -> impl IntoResponse {
    let mut data = ShareData::new();
    while let Some(field) = multipart.next_field().await.unwrap() {
        match field.name().unwrap() {
            "data" => {
                data.set_orig_name(field.file_name().unwrap().to_string());
                data.set_data(field.bytes().await.unwrap());
            }
            "expiration" => {
                let expiration = field.text().await.unwrap();
                data.set_expiration(
                    NaiveDate::parse_from_str(expiration.as_str(), "%Y-%m-%d").unwrap(),
                );
            }
            _ => {
                warn!("Found unknown data field");
            }
        }
    }

    match data.is_complete() {
        true => {
            data.write_to_disk().await;
            create_success_response(&data).await
        }
        false => create_error_response("No uploaded data found").await,
    }
}

pub async fn create_success_response(data: &ShareData) -> (axum::http::StatusCode, Html<String>) {
    if let Ok(template) = read_to_string("assets/upload_done.tpl.html").await {
        let final_document = template
            .replace("%filename%", data.get_orig_name())
            .replace("%size%", data.get_size().to_string().as_str())
            .replace("%id%", data.get_id());
        return (StatusCode::OK, Html(final_document));
    } else {
        // fail
    }
    create_error_response("Failed to perform templating").await
}

pub async fn create_error_response(msg: &str) -> (axum::http::StatusCode, Html<String>) {
    let mut retval = String::from(
        "<html><head><title>Teimeida Error</title></head><body><h1>Internal Server Error</h1><p>",
    );
    retval.push_str(msg);
    retval.push_str("</p></body></html>");
    (StatusCode::INTERNAL_SERVER_ERROR, Html(retval))
}
