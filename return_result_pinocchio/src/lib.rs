
use pinocchio::{
  AccountView, Address, ProgramResult, entrypoint, sysvars::instructions::Instructions
};
use solana_cpi::{get_return_data, invoke, set_return_data};
use solana_program_log::log;
use solana_instruction::Instruction;

mod normal_return;

entrypoint!(process_instruction);

pub fn process_instruction(
  program_id: &Address,
  accounts: &[AccountView],
  instruction_data: &[u8],
) -> ProgramResult {
  let program_to_call = &accounts[0];
  let instru = Instruction{
    program_id : *program_to_call.address(),
    accounts : vec![],
    data : vec![]
  };
  invoke(&instru, &[])?;
  let data = get_return_data().unwrap();
  
  Ok(())
}