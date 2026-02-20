mod instructions;

use pinocchio::{AccountView, Address, ProgramResult, entrypoint, error::ProgramError};

use crate::instructions::VaultInstruction;

const PROGRAM_ID: [u8; 32] = [0u8; 32];
entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Address,
    accounts: &[AccountView],
    data: &[u8],
) -> ProgramResult {
    if program_id.as_ref() != PROGRAM_ID {
        return Err(ProgramError::IncorrectProgramId);
    }
    let (discriminator, data) = data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;

    if data.len() < 8 {
        return Err(ProgramError::InvalidInstructionData);
    }
    let amount = u64::from_le_bytes(data[..8].try_into().unwrap());

    match VaultInstruction::try_from(*discriminator) {
        Ok(VaultInstruction::Deposit) => instructions::deposit::process(accounts, amount),
        Ok(VaultInstruction::Withdraw) => instructions::withdraw::process(accounts, amount),
        Err(_) => Err(ProgramError::InvalidInstructionData),
    }
}
