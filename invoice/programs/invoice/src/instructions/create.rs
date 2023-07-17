use std::str::FromStr;

use anchor_lang::prelude::*;
use crate::model::InvoiceError;
use crate::state::{Invoice};
use crate::constants::{AUTHORITY, INVOICE_SEED};

#[derive(Accounts)]
#[instruction(invoice_id: u64)]
pub struct Create<'info> {
    // For now, let's restrict to a certain authority
    // In the future, we can make this more flexible
    #[account(
        mut,
        address = Pubkey::from_str(AUTHORITY).unwrap() @ InvoiceError::UnauthorizedSigner
    )]
    pub authority: Signer<'info>,

    // PDA of the invoice
    #[account(
        init, 
        payer = authority, 
        space = Invoice::get_space(0), 
        seeds = [
            INVOICE_SEED.as_ref(),
            &(invoice_id).to_le_bytes()
            ], 
        bump
    )]
    pub invoice: Account<'info, Invoice>,

    #[account()]
    pub payer: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

pub fn create(ctx: Context<Create>, invoice_id: u64, description: String) -> Result<()> {
    let invoice = &mut ctx.accounts.invoice;
    let authority = &ctx.accounts.authority;
    let payer = &ctx.accounts.payer;

    // TODO @Aaron - incorporate require into .validate
    require!(description.len() <= Invoice::MAX_MEMO_LENGTH, InvoiceError::DescriptionTooLong);    

    invoice.initialize(
        authority.key(),
        payer.key(),
        *ctx.bumps.get("invoice").unwrap(),
        invoice_id,
        description
    );
    Ok(())
}
