use anchor_lang::prelude::*;
use solana_program::pubkey::Pubkey;

#[account]
pub struct Invoice {
    pub id: u64,
    pub bump: u8,
    pub creator: Pubkey,
    pub payer: Pubkey,
    pub payee: Pubkey, // for now this will be same as creator but in the future we can make this more flexible
    pub total_amount: u64,
    pub created_at: i64,
    pub due: i64,
    pub line_items: Vec<LineItem>,
    pub paid: bool, // TODO @Aaron TIME PERMITTING - make this an amount that can be updated (eg partial payments)
    pub state: InvoiceState,
    pub description: String, // TODO @Aaron TIME PERMITTING - make this a Vec of line items
    pub cancellation_reason: Option<String>, 
}

impl Invoice {
    pub const MAX_MEMO_LENGTH: usize = 100;
    pub const MAX_NUM_LINES: usize = 10;
    pub fn get_space(num_line_items: usize, cancelled: bool) -> usize {
        let mut space = 
            8 + // discriminator
            8 + // id
            1 + // bump
            32 + // creator
            32 + // payer
            32 + // payee
            8 + // created_at
            8 + // due
            4 + (LineItem::get_space() * num_line_items) + // Can I use Invoice::line_items.len()
            8 + // total_amount
            1 + // paid
            1 + 8 + // state // TODO @Aaron check space for enum https://book.anchor-lang.com/anchor_references/space.html
            4 + Self::MAX_MEMO_LENGTH; // description // TODO @Aaron make dynamic
    
        if cancelled {
            space += 100;
        }
    
        space
    }
    
    pub fn initialize(
        &mut self,
        auth: Pubkey,
        payer: Pubkey,
        bump: u8,
        invoice_id: u64,
        description: String,
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
        self.total_amount = 0;
    }

    pub fn add_item(&mut self, items: AddItemParams) {
        let line_item = LineItem {
            qty: items.qty,
            unit_cost: items.cost,
            item: items.item,
        };
        self.line_items.push(line_item);
    }
    pub fn send_invoice(&mut self) {
        self.state = InvoiceState::Unpaid;
        self.total_amount = self.line_items.iter().fold(0, |acc, item| {
            acc + item.calculate_total()
        });
        self.due = Clock::get().unwrap().unix_timestamp + 60 * 60 * 24 * 30; // 30 days from now
    }
    pub fn pay_invoice(&mut self) {
        self.state = InvoiceState::Paid;
        self.paid = true;
        // TODO Future: 
        // enable handling of SPL token payments to automate this (rather than a auth signature)
        // currently assuming the authorized user has received some off-chain payment and are using this to settle the invoice
        // Need to add a pentalty for late payments
    }
    pub fn cancel_invoice(&mut self, reason: String) {
        self.state = InvoiceState::Cancelled;
        self.cancellation_reason = Some(reason);
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone, PartialEq)]
pub enum InvoiceState {
    Unsent,
    Unpaid,
    Paid,
    Cancelled,
}

// TODO @Aaron TIME PERMITTING - add a new state for partially paid

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct LineItem {
    // Future state--could create a standard products list
    pub item: String, // Max 20 char
    pub qty: u8,
    pub unit_cost: u64,
}

impl LineItem {
    pub const MAX_ITEM_LENGTH: usize = 20;
    pub fn get_space() -> usize {
        4 + Self::MAX_ITEM_LENGTH + // item (String)
        1 + // qty (u8)
        8 // unit_cost (u64)
    }
    pub fn calculate_total(&self) -> u64 {
        self.qty as u64 * self.unit_cost
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct AddItemParams {
    pub qty: u8,
    pub cost: u64,
    pub item: String,
}
