use anyhow::Result;
use mpl_core::accounts::{BaseAssetV1, BaseCollectionV1};

pub fn deser_collection_header(bytes: &[u8]) -> Result<BaseCollectionV1> {
    Ok(BaseCollectionV1::from_bytes(bytes)?)
}

pub fn deser_asset_header(bytes: &[u8]) -> Result<BaseAssetV1> {
    Ok(BaseAssetV1::from_bytes(bytes)?)
}

pub fn ser_collection_header(header: &BaseCollectionV1) -> Result<Vec<u8>> {
    Ok(borsh::to_vec(header)?)
}

pub fn ser_asset_header(header: &BaseAssetV1) -> Result<Vec<u8>> {
    Ok(borsh::to_vec(header)?)
}
