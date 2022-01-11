use anchor_lang::prelude::*;

declare_id!("38RL3d6gUoz7rB5vMkfERNBQm9guJjvwR8GYVjQcEoBB");

#[program]
mod anchor_hello_world {
    use super::*;

    pub fn initializean(ctx: Context<Initialize>, authority: Pubkey) -> ProgramResult {
        let counter: &mut Account<Counter> = &mut ctx.accounts.counter;
        counter.authority = authority;
        counter.count = 0;
        Ok(())
    }

    pub fn increment(ctx: Context<Increment>) -> ProgramResult {
        let counter: &mut Account<Counter> = &mut ctx.accounts.counter;
        counter.count += 1;
        Ok(())
    }

    pub fn close_account(_ctx: Context<CloseAccount>) -> ProgramResult {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 40)]
    pub counter: Account<'info, Counter>,
    #[account(mut)]
    pub user: Signer<'info>,
    // Only needed when creating an account
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut, has_one = authority)]
    pub counter: Account<'info, Counter>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct CloseAccount<'info> {
    #[account(
        mut, 
        // close the account when the instruction end
        close = authority,
        has_one = authority,
    )]
    pub counter: Account<'info, Counter>,
    pub authority: AccountInfo<'info>,
}

#[account]
pub struct Counter {
    pub authority: Pubkey,
    pub count: u64,
}