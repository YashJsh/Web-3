use anchor_lang::prelude::*;

declare_id!("LZcsmFqHgDWkUzu82vgLQqXb6m73f21JquAyNfSnsdR");

#[program]
pub mod vault_anchor {
    use anchor_lang::system_program::{Transfer, transfer};

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.vault_state.vault_bump = ctx.bumps.vault;
        ctx.accounts.vault_state.state_bump = ctx.bumps.vault_state;
        Ok(())
    }

    pub fn deposit(ctx : Context<Operations>, amount : u64) -> Result<()>{
        let cpi_program = ctx.accounts.system_program.to_account_info();
        let cpi_accounts = Transfer{
            from : ctx.accounts.user.to_account_info(),
            to : ctx.accounts.vault.to_account_info()
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        let _ = transfer(cpi_ctx, amount);
        Ok(())
    }

    pub fn widthraw(ctx : Context<Operations>, amount : u64) -> Result<()>{
        let cpi_program = ctx.accounts.system_program.to_account_info();
        let cpi_accounts = Transfer{
            from : ctx.accounts.vault.to_account_info(),
            to : ctx.accounts.user.to_account_info()
        };
        let vault_address = ctx.accounts.vault_state.key();
        let seeds = &[
            b"vault",
            vault_address.as_ref(),
            &[ctx.accounts.vault_state.vault_bump]
        ];
        let signer_seeds = &[&seeds[..]];
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        let _ = transfer(cpi_ctx, amount);
        Ok(())
    }

    pub fn close(ctx : Context<Close>)-> Result<()>{
        let cpi = ctx.accounts.system_program.to_account_info();

        let cpi_accounts = Transfer{
            from : ctx.accounts.vault.to_account_info(),
            to : ctx.accounts.user.to_account_info()
        };
        let vault_address = ctx.accounts.vault_state.key();
        let seeds = &[
            b"vault",
            vault_address.as_ref(),
            &[ctx.accounts.vault_state.vault_bump]
        ];
        let signer_seeds = &[&seeds[..]];
        let cpi_ctx = CpiContext::new_with_signer(cpi, cpi_accounts, signer_seeds);

        let _ = transfer(cpi_ctx, ctx.accounts.vault.lamports());


        Ok(())
    }
}


#[derive(InitSpace)]
#[account]
pub struct VaultState{
    pub vault_bump : u8,
    pub state_bump : u8

}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user : Signer<'info>,

    #[account(
        init,
        payer = user,
        seeds = [b"state", user.key.as_ref()],
        bump,
        space = VaultState::INIT_SPACE

    )]
    pub vault_state : Account<'info, VaultState>,

    #[account(
        seeds = [b"vault", vault_state.key().as_ref()],
        bump,
    )]
    pub vault: SystemAccount<'info>,

    pub system_program : Program<'info, System>
}


#[derive(Accounts)]
pub struct Operations<'info>{
    #[account(mut)]
    user : Signer<'info>,

    #[account(
        mut, 
        seeds = [b"vault", vault_state.key().as_ref()],
        bump = vault_state.vault_bump,
    )]
    pub vault: SystemAccount<'info>,

    #[account(
        seeds = [b"state", user.key.as_ref()],
        bump = vault_state.state_bump,
    )]
    pub vault_state: Account<'info, VaultState>,

    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
pub struct Close<'info>{
    #[account(mut)]
    user : Signer<'info>,

    #[account(
        mut, 
        seeds = [b"vault", vault_state.key().as_ref()],
        bump = vault_state.vault_bump,
    )]
    pub vault: SystemAccount<'info>,

    #[account(
        mut,
        seeds = [b"state", user.key.as_ref()],
        bump = vault_state.state_bump,
        close = user
    )]
    pub vault_state: Account<'info, VaultState>,

    pub system_program: Program<'info, System>,
}