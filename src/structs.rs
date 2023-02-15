use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Media {
    pub(crate) url: String,
    pub(crate) originalUrl: String,
    pub(crate) thumbnailUrl: String,
    pub(crate) fileType: String,
    pub(crate) fileSize: u64,
}

#[derive(Debug, Deserialize)]
pub struct NftsResponse {
    pub(crate) identifier: String,
    pub(crate) collection: String,
    pub(crate) attributes: String,
    pub(crate) nonce: u64,
    pub(crate) name: String,
    pub(crate) creator: String,
    pub(crate) royalties: u64,
    pub(crate) uris: Vec<String>,
    pub(crate) url: String,
    pub(crate) media: Vec<Media>,
    pub(crate) isWhitelistedStorage: bool,
    pub(crate) tags: Vec<String>,
    pub(crate) ticker: String,
    pub(crate) isNsfw: bool,
}