use anyhow::Result;
use base64::{Engine, engine::general_purpose};

pub fn b64_to_bytes(b64: &str) -> Result<Vec<u8>> {
    Ok(general_purpose::STANDARD.decode(b64)?)
}

pub fn bytes_to_hex(bytes: &[u8]) -> Result<String> {
    Ok(format!("{}", hex::encode(bytes)))
}
