use bytes::Bytes;
use chrono::{Days, NaiveDate, Utc};
use nanoid::nanoid;
use tokio::{fs::File, io::AsyncWriteExt};

pub struct ShareData {
    data: Bytes,
    orig_name: String,
    storage_path: String,
    id: String,
    expiration: NaiveDate,
}

impl ShareData {
    pub fn new() -> Self {
        let data_id = nanoid!();
        ShareData {
            id: data_id.clone(),
            data: Bytes::new(),
            orig_name: String::new(),
            storage_path: "data/".to_owned() + &data_id,
            expiration: Utc::now()
                .checked_add_days(Days::new(14))
                .unwrap()
                .date_naive(),
        }
    }

    pub fn get_expiration(&self) -> &NaiveDate {
        &self.expiration
    }

    pub fn set_expiration(&mut self, expiration: NaiveDate) {
        self.expiration = expiration;
    }

    pub fn get_id(&self) -> &String {
        &self.id
    }

    pub fn get_orig_name(&self) -> &String {
        &self.orig_name
    }

    pub fn get_size(&self) -> usize {
        self.data.len()
    }

    pub fn set_data(&mut self, data: Bytes) {
        self.data = data;
    }

    pub fn set_orig_name(&mut self, orig_name: String) {
        self.orig_name = orig_name;
    }

    pub fn is_complete(&self) -> bool {
        !self.data.is_empty() && !self.orig_name.is_empty()
    }

    pub async fn write_to_disk(&self) -> bool {
        if !self.is_complete() {
            return false;
        }

        if let Ok(mut file) = File::create(&self.storage_path).await {
            if let Ok(_) = file.write_all(&self.data).await {
                // save meta-data as extended attributes
                let sp_result = xattr::set(
                    &self.storage_path,
                    "user.teimeida.orig_name",
                    &self.orig_name.as_bytes(),
                );
                let exp_result = xattr::set(
                    &self.storage_path,
                    "user.teimeida.expiration",
                    &self.expiration.to_string().as_bytes(),
                );
                sp_result.is_ok() && exp_result.is_ok()
            } else {
                false
            }
        } else {
            false
        }
    }
}
