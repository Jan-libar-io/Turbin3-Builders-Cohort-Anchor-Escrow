use anchor_lang::prelude::*;

declare_id!("8ffTQaLBEGtSAXGKaNpR1UhrftW6MK2gyWT3qXRUimX3");

#[program]
pub mod anchor_escrow {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
