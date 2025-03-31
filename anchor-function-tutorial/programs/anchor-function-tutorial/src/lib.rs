use anchor_lang::prelude::*;

declare_id!("HN2ztaK2FfRT8i75WCGudcEdNShBDVevgF7rRyGrBEBM");

#[program]
pub mod anchor_function_tutorial {
    use super::*;

    pub fn boaty_mc_boatface(ctx: Context<Initialize>) -> Result<()> {
        // number;
        // msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn add(ctx: Context<Initialize>, a: u64, b: u64) -> Result<()> {
        let sum = a + b;
        msg!("The sum is {}", sum);
        Ok(())
    }
    pub fn subtract(ctx: Context<Initialize>, c: u64, d:u64) -> Result<()> {
        let difference = c - d;
        msg!("The difference is {}", difference);
        Ok(())
    }

    pub fn multiply(ctx: Context<Initialize>, e: u64, f:u64) -> Result<()> {
        let product = e * f;
        msg!("The product is {}", product);
        Ok(())
    }
    pub fn div(ctx: Context<Initialize>, g: u64, h: u64) -> Result<()> {
        let quotient = g / h;
        msg!("The quotient is {}", quotient);
        Ok(())
    }
    pub fn non_empty_account_example(ctx: Context<NonEmptyAccountExample>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
pub struct NonEmptyAccountExample<'info> {
    signer: Signer<'info>,
    another_signer: Signer<'info>,
}
