use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::{entrypoint, ProgramResult},
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

#[derive(BorshDeserialize, BorshSerialize)]
pub enum CounterInstruction {
    Initialize,
    Increment,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Counter {
    pub count: u64,
}

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    account: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instructions = CounterInstruction::try_from_slice(instruction_data)?;

    match instructions {
        CounterInstruction::Initialize => {
            let counter_account = next_account_info(&mut account.iter())?;
            if counter_account.owner != program_id {
                return Err(ProgramError::IncorrectProgramId);
            }
            let counter = Counter { count: 0 };
            counter.serialize(&mut *counter_account.data.borrow_mut())?;
        }
        CounterInstruction::Increment => {
            let counter_account = next_account_info(&mut account.iter())?;

            let mut counter = Counter::try_from_slice(&counter_account.data.borrow_mut())?;
            counter.count += 1;
            counter.serialize(&mut *counter_account.data.borrow_mut())?;
        }
    }

    msg!("Instruction successfull");

    Ok(())
}
