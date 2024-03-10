use substreams::log::info;

pub fn account_as_string(account_list: Vec<Vec<u8>>, instruction_accounts: Vec<u8>, index: usize) -> String {
    // info!("account_list={:?}\n", account_list);
    // info!("instruction_accounts={:?}\n", instruction_accounts);
    // info!("index={:?}\n", instruction_accounts[index]);
    //
    info!("number={}", instruction_accounts[index] );
    if instruction_accounts[index] as usize >= account_list.len()
    {
        return "unknown".to_string();
    } else {
        bs58::encode(&account_list[
            instruction_accounts[index] as usize
            ]).into_string()
    }
}