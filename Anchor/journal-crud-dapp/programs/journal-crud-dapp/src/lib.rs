use anchor_lang::{accounts::signer, prelude::*};

declare_id!("FTz62VniwhUpjaupCoJobPkdsuRdZpAhGccxvJafD3oj");

#[program]
pub mod journal_crud_dapp {
    use super::*;
    pub fn create_journal_entry(ctx: Context<JournalEntryAccount>, title : String, message : String) -> Result<()> {
        let journal_entry = &mut ctx.accounts.journal_entry_account;
        journal_entry.owner = *ctx.accounts.signer.key;
        journal_entry.title = title;
        journal_entry.message  = message;
        
        Ok(())
    }
    pub fn update_journal_entry(ctx : Context<UpdateEntry>, title : String,  message : String)-> Result<()>{
        let update_entry = &mut ctx.accounts.journal_entry_account;
        update_entry.message = message;
        Ok(())
    }
    pub fn delete_journal_entry(_ctx : Context<DeleteEntry>, title : String)-> Result<()>{
        Ok(())
    }       
}

#[derive(Accounts)]
#[instruction(title : String)]
pub struct JournalEntryAccount<'info>{
    #[account(mut)]
    pub signer : Signer<'info>,

    #[account(
        init,
        payer = signer,
        seeds = [title.as_bytes(), signer.key().as_ref()],
        bump,
        space = 8 + JournalEntryState::INIT_SPACE
    )]
    pub journal_entry_account : Account<'info, JournalEntryState>,
    pub system_program : Program<'info, System>
}

#[derive(Accounts)]
#[instruction(title : String)]
pub struct UpdateEntry<'info>{
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [title.as_bytes(), signer.key().as_ref()],
        bump,
        realloc = 8 + JournalEntryState::INIT_SPACE,
        realloc::payer = signer,
        realloc::zero = false
    )]
    pub journal_entry_account : Account<'info, JournalEntryState>,

    pub system_program : Program<'info, System>
}

#[derive(Accounts)]
#[instruction(title : String)]
pub struct DeleteEntry<'info>{
    #[account(mut)]
    pub signer : Signer<'info>,

    #[account(
        mut,
        close = signer,
        seeds = [title.as_bytes(), signer.key().as_ref()],
        bump,
    )]
    pub delete_journal_account : Account<'info, JournalEntryState>,
    pub system_program : Program<'info, System>
}

#[derive(InitSpace)]
#[account]  
pub struct JournalEntryState{
    owner : Pubkey,
    #[max_len(50)]
    title : String,
    #[max_len(1000)]
    message : String
}