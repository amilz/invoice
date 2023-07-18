use std::str::FromStr;

use anchor_lang::prelude::*;
use crate::model::InvoiceError;
use crate::state::{Invoice, InvoiceState};
use crate::constants::{AUTHORITY, INVOICE_SEED};

#[derive(Accounts)]
pub struct CancelInvoice<'info> {
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
        bump = invoice.bump,
        realloc = Invoice::get_space((invoice.line_items.len()) as usize, true),
        realloc::payer = authority,
        realloc::zero = false,
    )]
    pub invoice: Account<'info, Invoice>,

    pub system_program: Program<'info, System>,
}


#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct CancelParams {
    reason: String
}



pub fn cancel_invoice(ctx: Context<CancelInvoice>, params: CancelParams) -> Result<()> {
    let invoice = &mut ctx.accounts.invoice;

    // TODO @Aaron - incorporate require into .validate
    require!(invoice.state == InvoiceState::Unpaid, InvoiceError::InvoiceNotOutstanding);

    invoice.cancel_invoice(params.reason);
    Ok(())
}
