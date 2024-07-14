use anchor_lang::prelude::*;

declare_id!("fg8jwjQaTebokzZBE5YWxFeRGuSZeVv1LW7oF4PPiu9");

#[program]
pub mod counter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
