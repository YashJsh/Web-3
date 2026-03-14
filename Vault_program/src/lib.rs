use pinocchio::{
    AccountView, Address, ProgramResult,
    cpi::{Seed, Signer},
    entrypoint,
    error::ProgramError,
    sysvars::{Sysvar, rent::Rent},
};
use solana_program_log::log;
mod account;

struct Vault {
    owner: [u8; 32],
    bump: u8,
}

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Address,
    accounts: &[AccountView],
    instruction_data: &[u8],
) -> ProgramResult {
    let (discriminator, data) = instruction_data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;

    match discriminator {
        0 => initalize_account(accounts, data, program_id),
        _ => Err(ProgramError::InvalidInstructionData),
    }
}
fn initalize_account(accounts: &[AccountView], data: &[u8], program_id: &Address) -> ProgramResult {
    let bump = data[0];
    let accounts = account::InitializeAccounts::try_from(accounts, bump, program_id)?;

    let space = core::mem::size_of::<Vault>();
    let rent = Rent::get()?;
    let min_rent = rent.minimum_balance(space);

    let binding = [bump];

    let seed1 = Seed::from(b"vault");
    let seed2 = Seed::from(accounts.signer.address().as_ref());
    let seed3 = Seed::from(&binding);
    let seeds = [seed1, seed2, seed3];

    let pda_signer = Signer::from(&seeds);

    //Now we will create this pda account;
    pinocchio_system::instructions::CreateAccount {
        from: accounts.signer,
        to: accounts.vault,
        lamports: min_rent,
        space: space as u64,
        owner: program_id,
    }
    .invoke_signed(&[pda_signer])?;

    let vault_state = Vault {
        bump: bump,
        owner: *accounts.signer.address().as_array(),
    };

    let mut vault_data = accounts.vault.try_borrow_mut()?;
    vault_data[0..32].copy_from_slice(accounts.signer.address().as_array());
    vault_data[4] = bump;
    log("Account vault initialized");
    Ok(())
}
