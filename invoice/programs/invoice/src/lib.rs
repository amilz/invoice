use anchor_lang::prelude::*;
pub mod id;
pub mod instructions;
pub mod model;
pub mod state;
pub mod constants;
pub use id::ID;
use instructions::*;


#[program]
pub mod invoice {
    use super::*;

    pub fn create(ctx: Context<Create>, invoice_id: u64, description: String) -> Result<()> {
        instructions::create(ctx, invoice_id, description)
    }
}

#[derive(Accounts)]
pub struct Initialize {}
