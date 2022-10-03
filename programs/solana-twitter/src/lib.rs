use anchor_lang::prelude::*;

declare_id!("CboCzBjAcsxz4yQtFYpJTuTCgmb1rQ9SqXBiMJ5x5ySC");

#[program]
pub mod solana_twitter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
