use borsh::BorshDeserialize;

#[derive(BorshDeserialize, Debug)]
pub struct TransferAmounts {
    pub amount: u64,
    pub decimals: u8,
}