use borsh::{BorshDeserialize, BorshSerialize};
use pinocchio::{
    AccountView, Address, ProgramResult, cpi::Signer, entrypoint, error::ProgramError, sysvars::{Sysvar, rent::Rent}
};
use pinocchio_pubkey::derive_address;


#[derive(BorshDeserialize, BorshSerialize)]
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
    let [signer, vault, system_program] = accounts else {
        return Err(ProgramError::InvalidAccountData);
    };

    if !signer.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    };

    //&[&[u8]; N]
    let bump = data[0];
    let seeds = &[b"vault", signer.address().as_ref(), &[bump]];
    let pda_vault = derive_address(seeds, Some(bump), program_id.as_array());
    if !pda_vault.eq(vault.address().as_array()) {
        return Err(ProgramError::InvalidAccountData);
    }
    let space = core::mem::size_of::<Vault>();
    let rent = Rent::get()?;
    let min_rent = rent.minimum_balance(space);

    //Now we will create this pda account;
    let new_account_instruction = pinocchio_system::instructions::CreateAccount {
        from: signer,
        to: vault,
        lamports: min_rent,
        space: space as u64,
        owner: program_id,
    };
    new_account_instruction.invoke_signed(&[Signer::try_from(signer)?])?;
    Ok(())
}
