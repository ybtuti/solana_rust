use anchor_lang::prelude::*;

declare_id!("DytQj3UuYF2N5hkESsq6KzFyNBx9rdHYhUCrpwRbEbaT");

#[program]
pub mod day1 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Hello, world!");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
