
use pinocchio::{
  AccountView, Address, ProgramResult, cpi::set_return_data, entrypoint
};
use solana_program_log::log;

entrypoint!(process_instruction);

pub fn process_instruction(
  program_id: &Address,
  accounts: &[AccountView],
  instruction_data: &[u8],
) -> ProgramResult {
  log!("Hello from my pinocchio program!");
  //Return success. 
  let data = "yashIsaChampion".as_bytes();
  set_return_data(data);
  Ok(())
}