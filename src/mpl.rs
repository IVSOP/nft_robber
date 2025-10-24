use anyhow::Result;
use mpl_core::accounts::{BaseAssetV1, BaseCollectionV1};

use crate::utils::*;

pub fn deser_collection(data: &str) -> Result<BaseCollectionV1> {
    let bytes = b64_to_bytes(data)?;
    Ok(BaseCollectionV1::from_bytes(&bytes)?)
}

pub fn deser_asset(data: &str) -> Result<BaseAssetV1> {
    let bytes = b64_to_bytes(data)?;
    Ok(BaseAssetV1::from_bytes(&bytes)?)
}

pub fn ser_collection_to_hex(collection: &BaseCollectionV1) -> Result<String> {
    let bytes = borsh::to_vec(collection)?;
    bytes_to_hex(&bytes)
}

pub fn ser_asset_to_hex(asset: &BaseAssetV1) -> Result<String> {
    let bytes = borsh::to_vec(asset)?;
    bytes_to_hex(&bytes)
}
