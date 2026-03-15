use borsh::{BorshDeserialize, BorshSerialize};
use bytemuck::{Pod, Zeroable};
use pinocchio::{
    AccountView, Address, ProgramResult,
    cpi::{Seed, Signer},
    entrypoint,
    error::ProgramError,
    sysvars::{Sysvar, rent::Rent},
};
use pinocchio_system::instructions::Transfer;
use solana_program_log::log;
mod account;

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct Vault {
    discriminator : u8,
    owner: [u8; 32],
    bump: u8,
}

#[derive(BorshDeserialize, BorshSerialize)]
struct Test{
    number : u64,
    authority : Option<[u8; 32]>,
    text : String
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
        1 => trasfer_vault(accounts, data, program_id),
        2 => trasfer_from_vault(accounts, data, program_id),
        4 => {
            log!("Testing instruction");
            if data.len() < 8 {
                log!("Data too short: {}", data.len());
                return Err(ProgramError::InvalidInstructionData);
            }
             let deserialize_data =
            Test::try_from_slice(data).map_err(|_| ProgramError::InvalidInstructionData)?;
            
            let number = deserialize_data.number;
            if deserialize_data.authority.is_some(){
                log!("Authority is present");
            }else{
                log!("No authority present");
            };
            let text = deserialize_data.text;
            log!("Text is : {}", text.as_str());
            
            Ok(())
        }
        _ => Err(ProgramError::InvalidInstructionData),
    }
}

fn initalize_account(accounts: &[AccountView], data: &[u8], program_id: &Address) -> ProgramResult {
    let bump = data[0];
    let accounts = account::FilterAccounts::try_from(accounts, bump, program_id)?;

    let vault_data = accounts.vault.try_borrow()?;
    
    if vault_data[0] != 0{
        return Err(ProgramError::AccountAlreadyInitialized);
    }

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
        discriminator : 1,
        bump: bump,
        owner: *accounts.signer.address().as_array(),
    };

    let mut vault_data = accounts.vault.try_borrow_mut()?;
    let dst = &mut vault_data[..std::mem::size_of::<Vault>()];
    dst.copy_from_slice(bytemuck::bytes_of(&vault_state));

    // vault_data[0..32].copy_from_slice(accounts.signer.address().as_array());
    // vault_data[4] = bump;
    log("Account vault initialized");
    Ok(())
}


fn trasfer_vault(accounts: &[AccountView], data: &[u8], program_id: &Address) -> ProgramResult {
    let accounts = account::FilterAccounts::try_from(accounts, data[0], program_id)?;
    let amount = u64::from_le_bytes(
    data[1..9]
        .try_into()
        .map_err(|_| ProgramError::InvalidInstructionData)?
    );

    let vault_data = accounts.vault.try_borrow()?;
    let vault_state: &Vault =
    bytemuck::from_bytes(&vault_data[..core::mem::size_of::<Vault>()]);

    if vault_state.owner != *accounts.signer.address().as_array(){
        return Err(ProgramError::InvalidAccountOwner)
    }
    Transfer{
        from : accounts.signer,
        to : accounts.vault,
        lamports : amount * 1_000_0000_000
    }.invoke()?;
    Ok(())
}

fn trasfer_from_vault(accounts: &[AccountView], data: &[u8], program_id: &Address) -> ProgramResult {
    let accounts = account::FilterAccounts::try_from(accounts, data[0], program_id)?;
    let amount = u64::from_le_bytes(
    data[1..9]
        .try_into()
        .map_err(|_| ProgramError::InvalidInstructionData)?
    );

    let binding = [data[0]];
    let seed1 = Seed::from(b"vault");
    let seed2 = Seed::from(accounts.signer.address().as_ref());
    let seed3 = Seed::from(&binding);
    let seeds = [seed1, seed2, seed3];

    let pda_signer = Signer::from(&seeds);

    let vault_data = accounts.vault.try_borrow()?;
    let vault_state: &Vault =
    bytemuck::from_bytes(&vault_data[..core::mem::size_of::<Vault>()]);

    if vault_state.owner != *accounts.signer.address().as_array(){
        return Err(ProgramError::InvalidAccountOwner)
    }
    Transfer{
        from : accounts.vault,
        to : accounts.signer,
        lamports : amount * 1_000_000_000
    }.invoke_signed(&[pda_signer])?;
    log!("Transfer executed successfully");
    Ok(())
}