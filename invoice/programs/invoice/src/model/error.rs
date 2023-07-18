use anchor_lang::prelude::*;

#[error_code]
pub enum InvoiceError {
    #[msg("Description must be less than 100 characters")]
    DescriptionTooLong,
    #[msg("Signer is not authorized to create invoices")]
    UnauthorizedSigner,
    #[msg("Cannot add more than 10 line items per invoice")]
    TooManyLineItems,
    #[msg("Item name is limited to 20 char")]
    ItemNameTooLong,
}