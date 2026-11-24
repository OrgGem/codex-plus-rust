//! Rust port of `services/share-site` (`_worker.js` + `app.js`).

#![forbid(unsafe_code)]

pub mod crypto;
pub mod markdown;
pub mod worker;

pub use crypto::{EncryptedPayload, EncryptedShare, decrypt_text, encrypt_text};
pub use markdown::{rollout_to_markdown, session_to_markdown};
pub use worker::{
    InMemoryShareStore, ShareRecord, ShareSiteConfig, ShareStore, WorkerRequest, WorkerResponse,
    handle_request,
};
