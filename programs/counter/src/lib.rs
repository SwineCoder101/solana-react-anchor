use anchor_lang::prelude::*;

declare_id!("EgqCXzzKvPEHuE3pMnKyKUWN33XJQLXrkqzCHaeu9cKg");

#[program]
pub mod counter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
