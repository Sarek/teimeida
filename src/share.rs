use axum::{extract::Multipart, response::Html};
use nanoid::nanoid;
use std::{
    fs::{self, File},
    io::Write,
};

pub async fn share_handler(mut multipart: Multipart) -> Html<String> {
    let id = nanoid!();
    let storage_path = "data/".to_owned() + &id;
    while let Some(field) = multipart.next_field().await.unwrap() {
        if field.name().unwrap() == "data" {
            if let Ok(mut file) = File::create(&storage_path) {
                let orig_name = field.file_name().unwrap().to_string();
                let data = field.bytes().await.unwrap();

                if let Ok(_) = file.write_all(&data) {
                    // save meta-data as extended attributes
                    let _ = xattr::set(
                        storage_path,
                        "user.teimeida.orig_name",
                        &orig_name.as_bytes(),
                    );
                    return create_success_response(&orig_name, &data.len().to_string(), &id).await;
                } else {
                    return create_error_response("Could not save all uploaded data").await;
                }
            } else {
                return create_error_response("Could not create storage file").await;
            }
        }
    }
    create_error_response("No uploaded data found").await
}

pub async fn create_success_response(
    filename: &String,
    size: &String,
    id: &String,
) -> Html<String> {
    if let Ok(template) = fs::read_to_string("assets/upload_done.tpl.html") {
        let final_document = template
            .replace("%filename%", filename)
            .replace("%size%", size)
            .replace("%id%", id);
        return Html(final_document);
    } else {
        // fail
    }
    Html("".to_string())
}

pub async fn create_error_response(msg: &str) -> Html<String> {
    let mut retval = String::from(
        "<html><head><title>Teimeida Error</title></head><body><h1>Internal Server Error</h1><p>",
    );
    retval.push_str(msg);
    retval.push_str("</p></body></html>");
    Html(retval)
}
