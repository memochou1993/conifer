use serde::Deserialize;

#[derive(Deserialize)]
pub struct ReqRecordStore {
    pub url: String,
    pub token: String,
}
