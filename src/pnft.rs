use anyhow::Result;
use mpl_token_metadata::{accounts::{Metadata, TokenRecord}, types::TokenState};
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
    const TOKEN_RECORD_SIZE: usize = 80;
    let mut data_vec: Vec<u8> = vec![0u8; TOKEN_RECORD_SIZE];
    // borsh::to_writer(&mut data_vec[..], token_record)?;
    // Ok(borsh::to_vec(token_record)?)
    // WHAT THE FUCK
    // borsh works differently for metaplex? wtf is going on?
    // is it because pnft uses a different version of borsh??????????
    // I'll just serialize it manually, whatever
    data_vec[0] = token_record.key as u8; // this can never go wrong. also this is just for testing
    data_vec[1] = token_record.bump;
    data_vec[2] = match token_record.state {
        TokenState::Unlocked => 0,
        TokenState::Locked => 1,
        TokenState::Listed => 2
    };

    if token_record.rule_set_revision.is_some() {
        panic!("got lazy, rule set revision is assumed to be none");
    }

    if token_record.delegate.is_some() {
        panic!("got lazy, delegate is assumed to be none");
    }

    if token_record.delegate_role.is_some() {
        panic!("got lazy, delegate role is assumed to be none");
    }

    if token_record.locked_transfer.is_some() {
        panic!("got lazy, locked transfer is assumed to be none");
    }

    data_vec[12..TOKEN_RECORD_SIZE].fill(0);

    Ok(data_vec)
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
    let tra = deser_token_record(bytes)?;
    println!("{:#?}", tra);
    Ok(())
}

pub fn print_metadata(bytes: &[u8]) -> Result<()> {
    let meta = deser_metadata(bytes)?;
    println!("{:#?}", meta);
    Ok(())
}
