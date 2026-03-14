use std::fmt::Error;

use pinocchio::{AccountView, Address, error::ProgramError};
use pinocchio_pubkey::derive_address;

pub struct InitializeAccounts<'a> {
    pub signer : &'a AccountView,
    pub vault : &'a AccountView,
    pub system_program : &'a AccountView
}

impl<'a>  InitializeAccounts<'a>{
    pub fn try_from(accounts: &'a [AccountView], bump : u8,  program_id: &Address) -> Result<Self, ProgramError> {
        let [signer, vault, system_program] = accounts else{
            return Err(ProgramError::InvalidInstructionData)
        };
        if !signer.is_signer() || !signer.is_writable(){
            return Err(ProgramError::MissingRequiredSignature);
        }

        let seeds = &[b"vault", signer.address().as_ref()];
        let vault_address = derive_address(seeds, Some(bump), program_id.as_array());

        if !vault_address.eq(vault.address().as_array()){
            return Err(ProgramError::InvalidAccountData)
        };

        Ok(Self { 
            signer, 
            vault, 
            system_program
        })
    }
}

