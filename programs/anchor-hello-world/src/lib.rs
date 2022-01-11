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

    // pub fn closeaccount(ctx: Context<Closeaccount>, authority: Pubkey) -> ProgramResult {
    //     let counter: &mut Account<Counter> = &mut ctx.accounts.counter;
    //     counter.authority = authority;
    //     counter.count = 0;
    //     Ok(())
    // }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 40)]
    pub counter: Account<'info, Counter>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut, has_one = authority)]
    pub counter: Account<'info, Counter>,
    pub authority: Signer<'info>,
}

// pub struct Closeaccount<'info> {
//     #[account(init, payer = user, space = 8 + 40)]
//     pub counter: Account<'info, Counter>,
//     pub user: Signer<'info>,
//     pub system_program: Program<'info, System>,
// }

#[account]
pub struct Counter {
    pub authority: Pubkey,
    pub count: u64,
}