use anchor_lang::prelude::*;
pub mod id;
pub mod instructions;
pub mod model;
pub mod state;
pub mod constants;
pub use id::ID;
use instructions::*;
use state::*;

#[program]
pub mod invoice {
    use super::*;

    pub fn create(ctx: Context<Create>, invoice_id: u64, description: String) -> Result<()> {
        instructions::create(ctx, invoice_id, description)
    }
    
    pub fn add_item(ctx: Context<AddItem>, invoice_id: u64, params: AddItemParams) -> Result<()> {
        instructions::add_item(ctx, invoice_id, params)
    }
}

#[derive(Accounts)]
pub struct Initialize {}
