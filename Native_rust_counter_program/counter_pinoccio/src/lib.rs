use borsh::{BorshDeserialize, BorshSerialize};
use pinocchio::{
    AccountView, Address, ProgramResult, entrypoint, error::ProgramError, sysvars::slot_hashes::log,
};
use pinocchio_system::instructions::CreateAccount;

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct Counter {
    count: u32,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub enum CounterProgram {
    Initialize,
    Increment,
}

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Address,
    accounts: &[AccountView],
    instruction_data: &[u8],
) -> ProgramResult {
    let instr = CounterProgram::try_from_slice(instruction_data)
        .map_err(|_| ProgramError::InvalidInstructionData)?;

    let mut counter_account = &accounts[0];

    let owner = unsafe {
        counter_account.owner()
    };
    if owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    };

    match instr{
        CounterProgram::Initialize => {
            let count = Counter{
                count : 0
            };

        },
        CounterProgram::Increment => {

        }
    }

  

    Ok(())
}

//compute units in a way is no. of instructions your program gonna execute.
//Anchor does like includes a lots of copies. That's why a cu is higher.
