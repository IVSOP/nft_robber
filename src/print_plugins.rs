use borsh::BorshDeserialize;
use mpl_core::accounts::{BaseAssetV1, BaseCollectionV1};
use mpl_core::types::{Key, Plugin};
use anyhow::Result;
use mpl_core::{fetch_collection_plugins, fetch_plugins, DataBlob};

// WARN: this is extremely inneficient and deserializes the same data over and over again. metaplex crate is not very good but I did not feel like manually deserializing stuff

pub fn print_asset_info(bytes: &[u8]) -> Result<()> {
    let key = Key::from_slice(bytes, 0)?;
    if ! matches!(key, Key::AssetV1) {
        anyhow::bail!("Is not an asset");
    }
    let asset_header = BaseAssetV1::from_bytes(bytes)?;

    println!("Asset header: {:#?}", asset_header);

    if bytes.len() == asset_header.len() {
        anyhow::bail!("No plugins found!");
    }

    let plugin_records = fetch_plugins(bytes)?;

    for record in plugin_records.iter() {
        println!("Record: {:#?}", record);
        // borsh deserialize moves the buffer to point at the new location
        // this is the worst possible thing that could have happened as we jump around given an offset
        // fuck this
        // I'll just constantly clone the bytes I don't even want to have any issues
        let bytes_clone_wtf = bytes.to_vec();
        let plugin = Plugin::deserialize(&mut &bytes_clone_wtf[record.offset as usize..])?;
        println!("Plugin: {:#?}", plugin);
    }

    Ok(())
}

pub fn print_collection_info(bytes: &[u8]) -> Result<()> {
    let key = Key::from_slice(bytes, 0)?;
    if ! matches!(key, Key::CollectionV1) {
        anyhow::bail!("Is not a collection");
    }
    let asset_header = BaseCollectionV1::from_bytes(bytes)?;

    println!("Asset header: {:#?}", asset_header);

    if bytes.len() == asset_header.len() {
        anyhow::bail!("No plugins found!");
    }

    // WARN: I made this function myself
    let plugin_records = fetch_collection_plugins(bytes)?;

    for record in plugin_records.iter() {
        println!("Record: {:#?}", record);
        // borsh deserialize moves the buffer to point at the new location
        // this is the worst possible thing that could have happened as we jump around given an offset
        // fuck this
        // I'll just constantly clone the bytes I don't even want to have any issues
        let bytes_clone_wtf = bytes.to_vec();
        let plugin = Plugin::deserialize(&mut &bytes_clone_wtf[record.offset as usize..])?;
        println!("Plugin: {:#?}", plugin);
    }

    Ok(())
}
