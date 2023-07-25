//use axum::extract::

use axum::{
    body::StreamBody,
    extract::Path,
    http::{header, HeaderMap, HeaderValue, StatusCode},
    response::IntoResponse,
};
use std::str;
use tokio_util::io::ReaderStream;

pub async fn retrieve_handler(Path(id): Path<String>) -> impl IntoResponse {
    let fullpath = format!("data/{}", id);
    let file = match tokio::fs::File::open(&fullpath).await {
        Ok(file) => file,
        Err(err) => return Err((StatusCode::NOT_FOUND, format!("File not found: {}", err))),
    };

    let orig_name = match xattr::get(&fullpath, "user.teimeida.orig_name") {
        Ok(orig_name) => orig_name.unwrap(),
        Err(err) => {
            return Err((
                StatusCode::NOT_FOUND,
                format!("Failed to read extended attributes: {}", err),
            ))
        }
    };

    let stream = ReaderStream::new(file);
    let body = StreamBody::new(stream);

    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        "application/octet-stream".parse().unwrap(),
    );
    let disposition = format!(
        "attachment; filename=\"{}\"",
        str::from_utf8(orig_name.as_slice()).unwrap()
    );
    headers.insert(
        header::CONTENT_DISPOSITION,
        HeaderValue::from_str(&disposition).unwrap(),
    );

    Ok((headers, body))
}
