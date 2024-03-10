use substreams::log::info;

pub fn account_as_string(account_list: Vec<Vec<u8>>, instruction_accounts: Vec<u8>, index: usize) -> String {
    bs58::encode(&account_list[
        instruction_accounts[index] as usize
        ]).into_string()
}