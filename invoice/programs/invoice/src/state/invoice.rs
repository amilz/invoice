use anchor_lang::prelude::*;
use solana_program::{pubkey::Pubkey};

#[account]
pub struct Invoice {
    pub id: u64, 
    pub bump: u8,
    pub creator: Pubkey,
    pub payer: Pubkey,
    pub payee: Pubkey, // for now this will be same as creator but in the future we can make this more flexible
    pub created_at: i64,
    pub due: u64,
    pub line_items: Vec<LineItem>,
    pub paid: bool, // TODO @Aaron TIME PERMITTING - make this a value that can be updated
    pub state: InvoiceState,
    pub description: String, // TODO @Aaron TIME PERMITTING - make this a Vec of line items
}


impl Invoice {
    pub const MAX_MEMO_LENGTH:usize = 100;
    pub const MAX_NUM_LINES: usize = 10;
    pub fn get_space(num_line_items: usize) -> usize {
        8 + // discriminator
        8 + // id
        1 + // bump
        32 + // creator
        32 + // payer
        32 + // payee
        8 + // created_at
        8 + // due
        4 + (LineItem::get_space() * num_line_items) + // Can I use Invoice::line_items.len()
        8 + // amount
        1 + // paid
        1 + 8 + // state // TODO @Aaron check space for enum https://book.anchor-lang.com/anchor_references/space.html
        4 + Self::MAX_MEMO_LENGTH // description // TODO @Aaron make dynamic
    }
    pub fn initialize(
        &mut self, 
        auth: Pubkey, 
        payer: Pubkey, 
        bump: u8, 
        invoice_id: u64,
        description: String
    ) {
        self.creator = auth;
        self.payer = payer;
        self.payee = auth; // TODO @Aaron enable this to be different from creator
        self.created_at = Clock::get().unwrap().unix_timestamp;
        self.due = 0; // we will set when we send the invoice
        self.line_items = Vec::new();
        self.paid = false;
        self.state = InvoiceState::Unsent;
        self.description = description;
        self.bump = bump;
        self.id = invoice_id;
    }

    
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub enum InvoiceState {
    Unsent,
    Unpaid,
    Paid,
    Cancelled,
}

// TODO @Aaron TIME PERMITTING - add a new state for partially paid

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct LineItem {
    pub item: String, // Max 20 char
    pub qty: u8, 
    pub unit_cost: u64,    
}

impl LineItem {
    const MAX_ITEM_LENGTH:usize = 20;
    pub fn get_space() -> usize {
        4 + Self::MAX_ITEM_LENGTH + // item (String)
        1 + // qty (u8)
        8 // unit_cost (u64)
    }
    pub fn calculate_total(self) -> u64 {        
        self.qty as u64 * self.unit_cost 
    }
}
