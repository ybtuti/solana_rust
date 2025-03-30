use anchor_lang::prelude::*;

declare_id!("Dn7c6Up37bYfxa3iNTmhLDe49pYquqvM7HqfB4emSiUR");

#[program]
pub mod day2 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, a: u64, b: u64, message: String) -> Result<()> {
        msg!("You said {:?}", message);
        msg!("You Sent {} and {}", a, b);
        Ok(())
    }
    pub fn array(ctx: Context<Initialize>, arr: Vec<u64>) -> Result<()> {
        msg!("Your array {:?}", arr);
        Ok(())
    }

    pub fn overflow_test(ctx: Context<Initialize>, y: u64, z: u64) -> Result<()> {
        let x: u64 = y - z;
        msg!("Your result is {}", x);
        Ok(())
    }
}


#[derive(Accounts)]
pub struct Initialize {}
