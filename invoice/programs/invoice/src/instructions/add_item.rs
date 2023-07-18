use std::str::FromStr;

use anchor_lang::prelude::*;
use crate::model::InvoiceError;
use crate::state::{Invoice, AddItemParams, LineItem, InvoiceState};
use crate::constants::{AUTHORITY, INVOICE_SEED};

#[derive(Accounts)]
#[instruction()]
pub struct AddItem<'info> {
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
            // TODO @Aaron time permitting change & test this from a parameter to using invoice.id
            &(invoice.id).to_le_bytes()
            ],
        realloc = Invoice::get_space(invoice.line_items.len() + 1, false),
        realloc::payer = authority,
        realloc::zero = false,
        bump = invoice.bump
    )]
    pub invoice: Account<'info, Invoice>,

    pub system_program: Program<'info, System>,
}


pub fn add_item(ctx: Context<AddItem>, params: AddItemParams) -> Result<()> {
    let invoice = &mut ctx.accounts.invoice;

    // TODO @Aaron - incorporate require into .validate
    require!(params.item.len() <= LineItem::MAX_ITEM_LENGTH, InvoiceError::DescriptionTooLong);    
    require!(invoice.line_items.len() + 1 < Invoice::MAX_NUM_LINES, InvoiceError::TooManyLineItems);
    require!(invoice.state == InvoiceState::Unsent, InvoiceError::InvoiceAlreadySent);

    invoice.add_item(params);
    
    Ok(())
}
