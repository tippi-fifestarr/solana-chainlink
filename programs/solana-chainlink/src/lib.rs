/* 
Defines a new anchor program
Creates an Decimal struct that defines how the price data is stored in the specified consumer account
Defines a fmt function for formatting price data with the correct decimals and zeros etc
Defines a new ‘execute’ function, which takes the following as inputs:
    The consumer account to store the price data
    The specified chainlink data feed account (eg SOL/USD) to obtain price data from
    The chainlink price feeds program account on Devnet
Defines the execute function body, which performs the following:
    Calls the `get_latest_round_data`, `description` and `decimals` functions of the chainlink data feed program for the specified price feed account
    Stores the result of the calls above in the specified consumer account via the Decimal struct
    Prints out the latest price to the program log output
Defines the execute function context, and what accounts are expected as input when it’s called */

use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program;

use chainlink_solana as chainlink;

declare_id!("5FqDJPHp1KvztoNwQUh3s3bHxqn99RbkCK7BnR61foMz");

#[account]
pub struct Decimal {
   pub value: i128,
   pub decimals: u32,
}

impl Decimal {
   pub fn new(value: i128, decimals: u32) -> Self {
       Decimal { value, decimals }
   }
}

impl std::fmt::Display for Decimal {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       let mut scaled_val = self.value.to_string();
       if scaled_val.len() <= self.decimals as usize {
           scaled_val.insert_str(
               0,
               &vec!["0"; self.decimals as usize - scaled_val.len()].join(""),
           );
           scaled_val.insert_str(0, "0.");
       } else {
           scaled_val.insert(scaled_val.len() - self.decimals as usize, '.');
       }
       f.write_str(&scaled_val)
   }
}

#[program]
pub mod solana_chainlink {
   use super::*;
       pub fn execute(ctx: Context<Execute>) -> ProgramResult  {
       let round = chainlink::latest_round_data(
           ctx.accounts.chainlink_program.to_account_info(),
           ctx.accounts.chainlink_feed.to_account_info(),
       )?;

       let description = chainlink::description(
           ctx.accounts.chainlink_program.to_account_info(),
           ctx.accounts.chainlink_feed.to_account_info(),
       )?;

       let decimals = chainlink::decimals(
           ctx.accounts.chainlink_program.to_account_info(),
           ctx.accounts.chainlink_feed.to_account_info(),
       )?;

       // Set the account value
       let decimal: &mut Account<Decimal> = &mut ctx.accounts.decimal;
       decimal.value=round.answer;
       decimal.decimals=u32::from(decimals);

       // Also print the value to the program output
       let decimal_print = Decimal::new(round.answer, u32::from(decimals));
       msg!("{} price is {}", description, decimal_print);
       Ok(())
   }
}

#[derive(Accounts)]
pub struct Execute<'info> {
   #[account(init, payer = user, space = 100)]
   pub decimal: Account<'info, Decimal>,
   #[account(mut)]
   pub user: Signer<'info>,
   pub chainlink_feed: AccountInfo<'info>,
   pub chainlink_program: AccountInfo<'info>,
   #[account(address = system_program::ID)]
   pub system_program: AccountInfo<'info>,
}