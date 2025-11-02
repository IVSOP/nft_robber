use anyhow::Result;
use mpl_token_metadata::accounts::{Metadata, TokenRecord};
use spl_token::solana_program::program_pack::Pack;

type AssociatedTokenAccount = spl_token::state::Account;

pub fn deser_ata(bytes: &[u8]) -> Result<AssociatedTokenAccount> {
    Ok(AssociatedTokenAccount::unpack(bytes)?)
}

pub fn ser_ata(ata: &AssociatedTokenAccount) -> Result<Vec<u8>> {
    let mut vec = vec![0u8; AssociatedTokenAccount::get_packed_len()];
    AssociatedTokenAccount::pack(*ata, &mut vec)?;
    Ok(vec)
}

pub fn deser_token_record(bytes: &[u8]) -> Result<TokenRecord> {
    Ok(TokenRecord::safe_deserialize(bytes)?)
}

pub fn ser_token_record(token_record: &TokenRecord) -> Result<Vec<u8>> {
    Ok(borsh::to_vec(token_record)?)
}

pub fn deser_metadata(bytes: &[u8]) -> Result<Metadata> {
    Ok(Metadata::safe_deserialize(bytes)?)
}

pub fn _ser_metadata(metadata: &Metadata) -> Result<Vec<u8>> {
    Ok(borsh::to_vec(metadata)?)
}

pub fn print_ata(bytes: &[u8]) -> Result<()> {
    let ata = deser_ata(bytes)?;
    println!("{:#?}", ata);
    Ok(())
}

pub fn print_token_record(bytes: &[u8]) -> Result<()> {
    let ata = deser_token_record(bytes)?;
    println!("{:#?}", ata);
    Ok(())
}

pub fn print_metadata(bytes: &[u8]) -> Result<()> {
    let ata = deser_metadata(bytes)?;
    println!("{:#?}", ata);
    Ok(())
}
