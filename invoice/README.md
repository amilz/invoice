Description: create a system for creating and processing invoices.

## Setup

Fork repo
Create a AUTH wallet using `solana keygen`
Update the `constants.rs` with that AUTH
Update tests with that AUTH
Make sure network set to localnet 
`anchor test`

## Notes

Create_invoice
- line item
- customer
- unique id
- date
- due date

Impl
- calculate total
- calculate tax
- calculate shipping
- calculate grand total

Process_invoice
- Mark_paid

admin: iuCjUFRP4fM7K6yRRPRPoPdmseuqsVTtFTqdJuy2rYJ