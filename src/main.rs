use std::str::FromStr;

use crate::{
    mpl::*,
    pnft::{
        deser_ata, deser_token_record, print_ata, print_metadata, print_token_record, ser_ata,
        ser_token_record,
    },
    print_plugins::*,
    rpc::*,
    utils::*,
};
use anyhow::Result;
use clap::{Parser, Subcommand};
use log::warn;
use mpl_token_metadata::{
    accounts::{Metadata, TokenRecord},
    types::TokenState,
};
use solana_address::Address;
use solana_pubkey::Pubkey;
use spl_associated_token_account::get_associated_token_address;
use spl_token::solana_program::program_option::COption;

mod mpl;
mod pnft;
mod print_plugins;
mod rpc;
mod utils;

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
    #[command(about = "Rob a pNFT")]
    RobPNft {
        mint: String,
        old_owner: String,
        new_owner: String,
    },
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
            let metadata_account = Metadata::find_pda(&mint_key).0;

            println!("ATA is {}:", ata_addr);
            if let Some(account_info_response) = rpc.get_account_info(&ata_addr.to_string()).await?
            {
                // WARN: I assume data is [data, "base64"], and that the format is base64
                let ata_data = b64_to_bytes(&account_info_response.data[0])?;
                print_ata(&ata_data)?;
            } else {
                anyhow::bail!("ATA account did not exist!");
            }

            println!("TRA is {}:", token_record_account);
            if let Some(account_info_response) = rpc
                .get_account_info(&token_record_account.to_string())
                .await?
            {
                // WARN: I assume data is [data, "base64"], and that the format is base64
                let ata_data = b64_to_bytes(&account_info_response.data[0])?;
                print_token_record(&ata_data)?;
            } else {
                anyhow::bail!("ATA account did not exist!");
            }

            println!("Metadata is {}:", metadata_account);
            if let Some(account_info_response) =
                rpc.get_account_info(&metadata_account.to_string()).await?
            {
                // WARN: I assume data is [data, "base64"], and that the format is base64
                let ata_data = b64_to_bytes(&account_info_response.data[0])?;
                print_metadata(&ata_data)?;
            } else {
                anyhow::bail!("ATA account did not exist!");
            }
        }
        Commands::RobPNft {
            mint,
            old_owner,
            new_owner,
        } => {
            check_key_valid(&mint)?;
            check_key_valid(&old_owner)?;
            check_key_valid(&new_owner)?;

            let mint_addr = Address::from_str(&mint)?;
            let mint_key = Pubkey::new_from_array(mint_addr.to_bytes());

            let old_owner_addr = Address::from_str(&old_owner)?;
            let new_owner_addr = Address::from_str(&new_owner)?;

            let old_ata_addr = get_associated_token_address(&old_owner_addr, &mint_addr);
            let new_ata_addr = get_associated_token_address(&new_owner_addr, &mint_addr);
            let old_ata_key = Pubkey::new_from_array(old_ata_addr.to_bytes());
            let new_ata_key = Pubkey::new_from_array(new_ata_addr.to_bytes());

            let old_tra_key = TokenRecord::find_pda(&mint_key, &old_ata_key).0;
            let new_tra_key = TokenRecord::find_pda(&mint_key, &new_ata_key).0;

            let new_tra_pda = Pubkey::find_program_address(
                &[
                    b"metadata",
                    mpl_token_metadata::ID.as_array(),
                    mint_key.as_array(),
                    b"token_record",
                    new_ata_key.as_array(),
                ],
                &mpl_token_metadata::ID,
            );

            let old_ata_account = rpc
                .get_account_info(&old_ata_addr.to_string())
                .await?
                .expect("old_ata does not exist");
            let old_tra_account = rpc
                .get_account_info(&old_tra_key.to_string())
                .await?
                .expect("old_tra does not exist");

            println!("Old ATA: {}", old_ata_key);
            println!("New ATA: {}", new_ata_key);
            println!("Old TRA: {}", old_tra_key);
            println!("New TRA: {}", new_tra_key);

            // Close old ATA and TRA
            println!("Closing old accounts");
            rpc.close_account(&old_ata_key.to_string()).await?;
            rpc.close_account(&old_tra_key.to_string()).await?;

            // Create new ones. Everything is cloned except the `owner` and `delegate` field of the ATA
            println!("Deserializing ATA");
            // remove delegation from the ATA and set the owner to the new owner
            let mut ata_info = deser_ata(&b64_to_bytes(&old_ata_account.data[0])?)?;
            ata_info.owner = new_owner_addr;
            ata_info.delegate = COption::None;

            println!("Deserializing TRA");
            // completely unfreeze the pNFT (the ATA remains frozen), and remove delegation
            let mut tra_info = deser_token_record(&b64_to_bytes(&old_tra_account.data[0])?)?;
            tra_info.state = TokenState::Unlocked;
            tra_info.bump = new_tra_pda.1;
            tra_info.rule_set_revision = None;
            tra_info.delegate = None;
            tra_info.delegate_role = None;
            tra_info.locked_transfer = None;

            println!("Serializing ATA");
            let ata_bytes = ser_ata(&ata_info)?;
            println!("Serializing TRA");
            let tra_bytes = ser_token_record(&tra_info)?;

            let debug_tra = deser_token_record(&tra_bytes)?;
            println!("debug: {:#?}", debug_tra);

            println!("Setting ATA");
            rpc.set_account_info(
                &new_ata_key.to_string(),
                &SetAccountInfo {
                    data: Some(bytes_to_hex(&ata_bytes)?),
                    executable: old_ata_account.executable,
                    lamports: old_ata_account.lamports,
                    owner: old_ata_account.owner,
                    rent_epoch: old_ata_account.rent_epoch,
                },
            )
            .await?;

            println!("Setting TRA");
            rpc.set_account_info(
                &new_tra_key.to_string(),
                &SetAccountInfo {
                    data: Some(bytes_to_hex(&tra_bytes)?),
                    executable: old_tra_account.executable,
                    lamports: old_tra_account.lamports,
                    owner: old_tra_account.owner,
                    rent_epoch: old_tra_account.rent_epoch,
                },
            )
            .await?;
        }
    }

    Ok(())
}
