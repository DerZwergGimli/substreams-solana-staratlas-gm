use std::fmt;
use borsh::BorshDeserialize;

pub const GM_PROGRAM: &str = "traderDnaR5w6Tcoi3NFm53i48FTDNbGjBSZwWXDRrg";

pub mod gm;
mod gm_accounts;
mod gm_args;
pub mod galactic_marketplace;
mod currencies;

#[derive(BorshDeserialize, Debug)]
pub struct Pubkey([u8; 32]);

impl fmt::Display for Pubkey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Convert the `[u8; 32]` to a hexadecimal string
        let string = bs58::encode(self.0.iter()).into_string();
        write!(f, "{}", string)
    }
}