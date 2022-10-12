use serde::Deserialize;

#[derive(Deserialize)]
pub struct ReqStoreRecord {
    pub url: String,
}
