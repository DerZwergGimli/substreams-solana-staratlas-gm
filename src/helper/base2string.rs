use substreams::log::info;

pub fn account_as_string(account_list: Vec<Vec<u8>>, instruction_accounts: Vec<u8>, index: usize) -> String {
    // info!("account_list={:?}\n", account_list.len());
    // info!("instruction_accounts={:?}\n", instruction_accounts.len());
    // info!("index={:?}\n", instruction_accounts[index]);

    if instruction_accounts.len() > index {
        if instruction_accounts.len() >= (instruction_accounts[index] as usize) {
            bs58::encode(&account_list[instruction_accounts[index] as usize]).into_string()
        } else {
            "unknown".to_string()
        }
    } else {
        "unknown".to_string()
    }
}