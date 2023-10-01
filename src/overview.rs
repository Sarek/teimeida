use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};
use chrono::{offset::Utc, DateTime, NaiveDate};
use std::{str::FromStr, time::SystemTime};
use tokio::fs::{read_dir, read_to_string, DirEntry};

pub async fn overview_handler() -> impl IntoResponse {
    match read_dir("data").await {
        Ok(mut dir_content) => {
            let mut overview_parts = String::new();
            let mut total_size = 0;
            while let Ok(Some(entry)) = dir_content.next_entry().await {
                let id = entry.file_name().to_str().unwrap().to_string();
                if let Ok(metadata) = entry.metadata().await {
                    let size = metadata.len();
                    let upload = metadata.created().unwrap_or(SystemTime::UNIX_EPOCH);
                    let upload: DateTime<Utc> = upload.into();
                    let upload = upload.format("%Y-%m-%d %H:%M:%S UTC");
                    if let Some((orig_name, expiration)) = get_xattr_data(&entry) {
                        total_size += size;
                        let part = format!("<tr><td><a href=\"retrieve/{}\">{}</a></td><td>{}</td><td>{}</td><td>{}</td><td class=\"right\">{}</td></tr>", &id, &id, &orig_name, &upload, &expiration, &as_mib(size));
                        overview_parts.push_str(&part);
                    } else {
                        return create_error_response(&format!(
                            "Failed to read extended attributes for {}",
                            id
                        ))
                        .await;
                    }
                } else {
                    return create_error_response(&format!("Failed to read metadata for {}", id))
                        .await;
                }
            }

            return create_success_response(&overview_parts, total_size).await;
        }
        Err(e) => {
            return create_error_response(&format!("Failed to create overview: {}", e)).await;
        }
    }
}

fn get_xattr_data(path: &DirEntry) -> Option<(String, NaiveDate)> {
    let orig_name = xattr::get(path.path(), "user.teimeida.orig_name");
    let expiration = xattr::get(path.path(), "user.teimeida.expiration");

    if orig_name.is_err() || expiration.is_err() {
        return Option::None;
    }

    let orig_name = String::from_utf8(orig_name.unwrap().unwrap()).unwrap();
    let expiration =
        NaiveDate::from_str(&String::from_utf8(expiration.unwrap().unwrap()).unwrap()).unwrap();

    Some((orig_name, expiration))
}

pub async fn create_success_response(
    data: &str,
    size: u64,
) -> (axum::http::StatusCode, Html<String>) {
    if let Ok(template) = read_to_string("templates/overview.tpl.html").await {
        let final_document = template
            .replace("%overview_rows%", data)
            .replace("%storage%", &as_mib(size));
        return (StatusCode::OK, Html(final_document));
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

pub fn as_mib(num: u64) -> String {
    format!("{:.2} MiB", (num as f64 / 1024_f64 / 1024_f64))
}
