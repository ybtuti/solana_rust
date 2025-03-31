use anchor_lang::prelude::*;

declare_id!("DX62UxLHVmpr8TtdibNc64EPdhV2J3ngow9S56PVmRzS");

#[program]
pub mod day4 {
    use super::*;

    // pub fn limit_range(ctx: Context<LimitRange>, a: u64) -> Result<()> {
    //     if a < 10 {
    //         return err!(MyError::AIsTooSmall);            
    //     }
    //     if a > 100 {
    //         return err!(MyError::AIsTooBig);
    //     }
    //     msg!("Result = {}", a);
    //     Ok(())
    // }
    pub fn func(ctx: Context<LimitRange>) -> Result<()> {
            msg!("Will this print?");
            return err!(MyError::AlwaysErrors);
        
    }

    pub fn limit_range(ctx: Context<LimitRange>, a: u64) -> Result<()> {
        require!(a >= 10, MyError::AIsTooSmall);
        require!(a <= 100, MyError::AIsTooBig);
    
        msg!("Result = {}", a);
        Ok(())
    }    

}

#[derive(Accounts)]
pub struct LimitRange {}

#[error_code]
pub enum MyError {
    #[msg("A is too big")]
    AIsTooBig,
    #[msg("A is too small")]
    AIsTooSmall,
    #[msg("Always errors")]  
    AlwaysErrors,
}