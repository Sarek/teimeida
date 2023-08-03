use chrono::{Local, NaiveDate};
use std::str::{self, FromStr};
use tokio::fs::{self, DirEntry};
use tokio_stream::{wrappers::ReadDirStream, StreamExt};

pub async fn cleanup() {
    info!("Executing periodic cleanup task");

    if let Ok(entries) = fs::read_dir("data").await {
        let mut entries_stream = ReadDirStream::new(entries);

        while let Some(entry) = entries_stream.next().await {
            if let Ok(entry) = entry {
                info!("Checking entry {:#?}", &entry.file_name());
                remove_if_expired(&entry).await.unwrap_or_else(|x| {
                    error!(
                        "Error during expiration check for {:#?}: {}",
                        entry.file_name(),
                        x
                    )
                });
            }
        }
    }
}

pub async fn remove_if_expired(entry: &DirEntry) -> Result<(), std::io::Error> {
    if !entry.file_type().await.unwrap().is_file() {
        return Result::Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Directory entry is not a file",
        ));
    }

    let expired = xattr::get(entry.path(), "user.teimeida.expiration")
        .and_then(|expiration| {
            expiration.ok_or(std::io::Error::new(
                std::io::ErrorKind::Other,
                "No expiration value found",
            ))
        })
        .map(is_expired)
        .unwrap_or(false);

    if expired {
        info!("Removing expired entry {:#?}", entry.file_name());
        return tokio::fs::remove_file(entry.path()).await;
    }

    return Result::Ok(());
}

fn is_expired(expiration: Vec<u8>) -> bool {
    str::from_utf8(&expiration.as_slice())
        .map(|x| {
            NaiveDate::from_str(x)
                .map(|x| x < Local::now().naive_local().date())
                .unwrap_or(false)
        })
        .unwrap_or(false)
}
