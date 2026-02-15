use pinocchio::{
    AccountView, Address, ProgramResult, entrypoint, error::ProgramError, sysvars::slot_hashes::log
};
use pinocchio_system::instructions::CreateAccount;

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id : &Address,
    accounts: &[AccountView],
    _instruction_data : &[u8]
)-> ProgramResult {
    let [from, to, system_program] = accounts else
        {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    

    let balance = from.lamports();
    
    CreateAccount{
        from : from,
        to : to,
        lamports : 1_000_000_000,
        space : 10,
        owner : program_id
    }.invoke()

    // core::hint::black_box(Ok(()))
}


 
//compute units in a way is no. of instructions your program gonna execute.
//Anchor does like includes a lots of copies. That's why a cu is higher.
