use std::str::FromStr;

use anchor_lang::prelude::*;
use crate::model::InvoiceError;
use crate::state::{Invoice, InvoiceState};
use crate::constants::{AUTHORITY, INVOICE_SEED};

#[derive(Accounts)]
#[instruction()]
pub struct SendInvoice<'info> {
    // For now, let's restrict to a certain authority
    // In the future, we can make this more flexible
    #[account(
        mut,
        address = Pubkey::from_str(AUTHORITY).unwrap() @ InvoiceError::UnauthorizedSigner
    )]
    pub authority: Signer<'info>,

    // PDA of the invoice
    #[account(
        mut, 
        seeds = [
            INVOICE_SEED.as_ref(),
            &(invoice.id).to_le_bytes()
            ], 
        bump = invoice.bump
    )]
    pub invoice: Account<'info, Invoice>,

    pub system_program: Program<'info, System>,
}


pub fn send_invoice(ctx: Context<SendInvoice>) -> Result<()> {
    let invoice = &mut ctx.accounts.invoice;

    // TODO @Aaron - incorporate require into .validate
    require!(invoice.line_items.len() + 1 < Invoice::MAX_NUM_LINES, InvoiceError::TooManyLineItems);
    require!(invoice.state == InvoiceState::Unsent, InvoiceError::InvoiceAlreadySent);

    invoice.send_invoice();
    
    Ok(())
}
