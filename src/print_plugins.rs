use borsh::BorshDeserialize;
use mpl_core::accounts::BaseAssetV1;
use mpl_core::types::{Key, Plugin};
use anyhow::Result;
use mpl_core::fetch_plugins;

// WARN: this is extremely inneficient and deserializes the same data over and over again. metaplex crate is not very good but I did not feel like manually deserializing stuff

pub fn print_asset_info(bytes: &[u8]) -> Result<()> {
    let key = Key::from_slice(bytes, 0)?;
    if ! matches!(key, Key::AssetV1) {
        anyhow::bail!("Wrong key");
    }
    let asset_header = BaseAssetV1::from_bytes(bytes)?;
    // let asset_header_len = asset_header.len();

    println!("Asset header: {:#?}", asset_header);

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
