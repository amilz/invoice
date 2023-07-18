use anchor_lang::prelude::*;
pub mod constants;
pub mod id;
pub mod instructions;
pub mod model;
pub mod state;
pub use id::ID;
use instructions::*;
use state::*;

#[program]
pub mod invoice {
    use super::*;

    pub fn create(ctx: Context<Create>, invoice_id: u64, description: String) -> Result<()> {
        instructions::create(ctx, invoice_id, description)
    }

    pub fn add_item(ctx: Context<AddItem>, params: AddItemParams) -> Result<()> {
        instructions::add_item(ctx, params)
    }

    pub fn send_invoice(ctx: Context<SendInvoice>) -> Result<()> {
        instructions::send_invoice(ctx)
    }

    pub fn process_payment(ctx: Context<ProcessPayment>) -> Result<()> {
        instructions::process_payment(ctx)
    }

    pub fn cancel_invoice(ctx: Context<CancelInvoice>, params: CancelParams) -> Result<()> {
        instructions::cancel_invoice(ctx, params)
    }
}

#[derive(Accounts)]
pub struct Initialize {}
