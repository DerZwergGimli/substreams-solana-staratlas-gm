use borsh::BorshDeserialize;
use crate::galactic_marketplace::Pubkey;

#[derive(BorshDeserialize, Debug)]
pub struct ProcessInitializeSellArgs {
    pub price: u64,
    pub origination_qty: u64,
}


#[derive(BorshDeserialize, Debug)]
pub struct ProcessExchangeArgNoPubkeyAndPrice {
    pub purchase_quantity: u64,
}

#[derive(BorshDeserialize, Debug)]
pub struct ProcessExchangeArgNoPubkey {
    pub purchase_quantity: u64,
    pub expected_price: u64,
}

#[derive(BorshDeserialize, Debug)]
pub struct ProcessExchangeArgsWithPubkey {
    pub purchase_quantity: u64,
    pub expected_price: u64,
    pub seller: Pubkey,
}