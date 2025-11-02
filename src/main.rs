use std::str::FromStr;

use crate::{mpl::*, pnft::{print_ata, print_token_record}, print_plugins::*, rpc::*, utils::*};
use anyhow::Result;
use clap::{Parser, Subcommand};
use log::warn;
use mpl_token_metadata::accounts::TokenRecord;
use solana_address::Address;
use solana_pubkey::{Pubkey, pubkey};
use spl_associated_token_account::get_associated_token_address;

mod mpl;
mod print_plugins;
mod rpc;
mod utils;
mod pnft;

/// Command line parser using `clap`
#[derive(Parser, Debug)]
#[command(name = "core_parser")]
#[command(about = "NFT robber")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[command(about = "Steal a core nft")]
    RobCoreNft { nft_key: String, new_owner: String },
    #[command(about = "Steal a core collection")]
    RobCoreCollection {
        collection_key: String,
        new_authority: String,
    },
    #[command(about = "Print information for a core NFT")]
    PrintCoreNft { key: String },
    #[command(about = "Print information for a core collection")]
    PrintCoreCollection { key: String },
    #[command(about = "Print information for a programmable collection")]
    PrintPNft { mint: String, owner: String },
}

// cursed
pub fn check_key_valid(key: &str) -> Result<()> {
    if Pubkey::from_str(&key).is_ok() {
        Ok(())
    } else {
        anyhow::bail!("{} is not a valid key", key);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    if dotenvy::dotenv().is_err() {
        warn!("Could not load a .env file");
    }
    env_logger::init();

    let rpc = Rpc::new("http://localhost:8899".into());

    let cli = Cli::parse();

    match cli.command {
        Commands::RobCoreNft { nft_key, new_owner } => {
            check_key_valid(&nft_key)?;
            check_key_valid(&new_owner)?;

            if let Some(account_info_response) = rpc.get_account_info(&nft_key).await? {
                // WARN: I assume data is [data, "base64"], and that the format is base64
                // another WARN: the deserialization the metaplex crate does will only deserialize the header
                // this means that if you just write the header you will be deleting all the other data in the NFT, like plugins
                let mut asset_data = b64_to_bytes(&account_info_response.data[0])?;
                let mut asset_header = deser_asset_header(&asset_data)?;
                asset_header.owner = Pubkey::from_str(&new_owner)?;

                // see the warning above. need to keep the remaining data intact, so just copy the header
                // the header length does not change when changing the owner etc
                let new_header_data = ser_asset_header(&asset_header)?;
                asset_data[..new_header_data.len()].copy_from_slice(&new_header_data);

                let data_string = bytes_to_hex(&asset_data)?;

                let set_account_info = SetAccountInfo {
                    data: Some(data_string),
                    executable: account_info_response.executable,
                    lamports: account_info_response.lamports,
                    owner: account_info_response.owner,
                    rent_epoch: account_info_response.rent_epoch,
                };

                rpc.set_account_info(&nft_key, &set_account_info).await?;
            } else {
                anyhow::bail!("NFT account did not exist!");
            }
        }
        Commands::RobCoreCollection {
            collection_key,
            new_authority,
        } => {
            check_key_valid(&collection_key)?;
            check_key_valid(&new_authority)?;

            if let Some(account_info_response) = rpc.get_account_info(&collection_key).await? {
                let mut collection_data = b64_to_bytes(&account_info_response.data[0])?;
                let mut collection_header = deser_collection_header(&collection_data)?;
                collection_header.update_authority = Pubkey::from_str(&new_authority)?;

                let new_header_data = ser_collection_header(&collection_header)?;
                collection_data[..new_header_data.len()].copy_from_slice(&new_header_data);

                let data_string = bytes_to_hex(&collection_data)?;

                let set_account_info = SetAccountInfo {
                    data: Some(data_string),
                    executable: account_info_response.executable,
                    lamports: account_info_response.lamports,
                    owner: account_info_response.owner,
                    rent_epoch: account_info_response.rent_epoch,
                };

                rpc.set_account_info(&collection_key, &set_account_info)
                    .await?;
            } else {
                anyhow::bail!("Collection account did not exist!");
            }
        }
        Commands::PrintCoreNft { key } => {
            check_key_valid(&key)?;

            if let Some(account_info_response) = rpc.get_account_info(&key).await? {
                // WARN: I assume data is [data, "base64"], and that the format is base64
                let asset_data = b64_to_bytes(&account_info_response.data[0])?;
                print_asset_info(&asset_data)?;
            } else {
                anyhow::bail!("NFT account did not exist!");
            }
        }
        Commands::PrintCoreCollection { key } => {
            check_key_valid(&key)?;

            if let Some(account_info_response) = rpc.get_account_info(&key).await? {
                // WARN: I assume data is [data, "base64"], and that the format is base64
                let asset_data = b64_to_bytes(&account_info_response.data[0])?;
                print_collection_info(&asset_data)?;
            } else {
                anyhow::bail!("NFT account did not exist!");
            }
        }
        Commands::PrintPNft { mint, owner } => {
            check_key_valid(&mint)?;
            check_key_valid(&owner)?;

            let mint_addr = Address::from_str(&mint)?;
            let mint_key = Pubkey::new_from_array(mint_addr.to_bytes());

            let owner_addr = Address::from_str(&owner)?;

            let ata_addr = get_associated_token_address(&owner_addr, &mint_addr);
            let ata_key = Pubkey::new_from_array(ata_addr.to_bytes());

            let token_record_account = TokenRecord::find_pda(&mint_key, &ata_key).0;

            println!("ATA is {}:", ata_addr);
            if let Some(account_info_response) = rpc.get_account_info(&ata_addr.to_string()).await? {
                // WARN: I assume data is [data, "base64"], and that the format is base64
                let ata_data = b64_to_bytes(&account_info_response.data[0])?;
                print_ata(&ata_data)?;
            } else {
                anyhow::bail!("ATA account did not exist!");
            }

            println!("TRA is {}:", token_record_account);
            if let Some(account_info_response) = rpc.get_account_info(&token_record_account.to_string()).await? {
                // WARN: I assume data is [data, "base64"], and that the format is base64
                let ata_data = b64_to_bytes(&account_info_response.data[0])?;
                print_token_record(&ata_data)?;
            } else {
                anyhow::bail!("ATA account did not exist!");
            }
        }
    }

    Ok(())
}
