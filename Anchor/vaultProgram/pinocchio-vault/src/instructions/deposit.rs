use pinocchio::{
    AccountView,
    error::ProgramError,
    ProgramResult,
};
use pinocchio_system::instructions::Transfer;

pub fn process(
    accounts : &[AccountView],
    lamports : u64
)-> ProgramResult{
    if accounts.len() < 2{
        return Err(ProgramError::NotEnoughAccountKeys)
    }

    let signer = &accounts[0];
    let vault = &accounts[1];

    //Validate signer
    if !signer.is_signer(){
        return Err(ProgramError::MissingRequiredSignature)
    }
    //check for writable
    if !signer.is_writable() || !vault.is_writable(){
        return Err(ProgramError::InvalidAccountData)
    }
    Transfer{
        from : signer,
        to : vault,
        lamports
    }
    .invoke()
}

