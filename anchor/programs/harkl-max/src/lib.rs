use anchor_lang::prelude::*;

declare_id!("HmdQhKEj9wC5ztUTPAqUtB1E1fqWYqgFxeNfPLDwXARu");

#[program]
pub mod harkl_max {
    use super::*;

    pub fn greet(_ctx: Context<Initialize>) -> Result<()> {
        msg!("GM!");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
