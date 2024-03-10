use std::collections::BTreeMap;
use anyhow::anyhow;
use borsh::BorshDeserialize;
use substreams::errors::Error;
use substreams::log::info;
use substreams_solana::pb::sf::solana::r#type::v1::{Block, CompiledInstruction, Transaction, TransactionStatusMeta};

use crate::galactic_marketplace::currencies::get_currency_decimals;
use crate::galactic_marketplace::gm_accounts::{PROCESS_EXCHANGE_ACCOUNTS_19, PROCESS_EXCHANGE_ACCOUNTS_28, PROCESS_EXCHANGE_ACCOUNTS_32, PROCESS_INITIALIZE_ACCOUNTS_14, PROCESS_INITIALIZE_ACCOUNTS_27};
use crate::galactic_marketplace::gm_args::{ProcessExchangeArgNoPubkey, ProcessExchangeArgNoPubkeyAndPrice, ProcessExchangeArgsWithPubkey, ProcessInitializeSellArgs};
use crate::helper::base2string::account_as_string;
use crate::lookup::{LOOKUP_TABLE, LOOKUP_TABLE_KEY};
use crate::pb::sa::gm::market::v1::galactic_marketplace_instruction::{Account, Arg, Instruction::*, MetaData};
use crate::pb::sa::gm::market::v1::GalacticMarketplaceInstruction;
use crate::pb::sol::token::program::v1::token_program::Program;
use crate::pb::sol::token::program::v1::TokenProgram;
use crate::solana_token_program::TOKEN_PROGRAM;

impl GalacticMarketplaceInstruction {
    pub fn unpack(block: Block, transaction: &Transaction, compiled_instruction: &CompiledInstruction, instruction_idx: usize, meta: TransactionStatusMeta) -> Result<Self, Error> {
        let (&tag, rest) = compiled_instruction.data.split_first().ok_or(anyhow!("Unable to split instruction data"))?;
        let (_dump, exchange_args) = rest.split_at(7);

        let meta_data = Some(MetaData {
            signature: bs58::encode(transaction.signatures[0].as_slice()).into_string(),
            timestamp: block.block_time.clone().unwrap().timestamp,
            block: block.slot,
        });
        let mut accounts = vec![];
        let mut args = vec![];
        let mut parsed = vec![];
        let mut inner_instructions = vec![];


        Ok(match tag {
            9 => {
                GalacticMarketplaceInstruction {
                    meta_data,
                    instruction: i32::from(UnknownTransaction),
                    accounts,
                    args,
                    parsed,
                    inner_instructions,
                }
            }
            12 => {
                GalacticMarketplaceInstruction {
                    meta_data,
                    instruction: i32::from(UnknownTransaction),
                    accounts,
                    args,
                    parsed,
                    inner_instructions,
                }
            }

            18 => {
                GalacticMarketplaceInstruction {
                    meta_data,
                    instruction: i32::from(UpdateCurrencyVault),
                    accounts,
                    args,
                    parsed,
                    inner_instructions,
                }
            }

            43 | 129 => {
                // ProcessInitializeSell and ProcessInitializeBuy
                match exchange_args.len() {
                    16 => {
                        let ProcessInitializeSellArgs { price, origination_qty } = ProcessInitializeSellArgs::try_from_slice(exchange_args)?;
                        {
                            args.push(Arg { name: "price".to_string(), r#type: "u64".to_string(), value: price.to_string() });
                            args.push(Arg { name: "origination_qty".to_string(), r#type: "u64".to_string(), value: origination_qty.to_string() });
                        }
                    }
                    _ => {
                        return Err(anyhow!("No exchange_args for ProcessInitializeSellArgs len for instruction: len={}", exchange_args.len()));
                    }
                }


                //MAPPING
                accounts = match compiled_instruction.accounts.len() {
                    14 => {
                        map_account_names(transaction.message.clone().unwrap().account_keys, compiled_instruction.accounts.clone(), &PROCESS_INITIALIZE_ACCOUNTS_14)
                    }
                    27 => {
                        map_account_names(transaction.message.clone().unwrap().account_keys, compiled_instruction.accounts.clone(), &PROCESS_INITIALIZE_ACCOUNTS_27)
                    }
                    _ => return Err(anyhow!("No exchange_args len for  compiled_instruction.accounts: for mapping len={}", compiled_instruction.accounts.len()))
                };

                inner_instructions = map_inner_instruction(transaction, instruction_idx, meta);

                match tag {
                    43 => {
                        GalacticMarketplaceInstruction {
                            meta_data,
                            instruction: i32::from(ProcessInitializeSell),
                            accounts,
                            args,
                            parsed,
                            inner_instructions,

                        }
                    }
                    129 => {
                        GalacticMarketplaceInstruction {
                            meta_data,
                            instruction: i32::from(ProcessInitializeBuy),
                            accounts,
                            args,
                            parsed,
                            inner_instructions,
                        }
                    }
                    _ => {
                        return Err(anyhow!("No tag for instruction: tag={}", tag));
                    }
                }
            }
            47 => {
                GalacticMarketplaceInstruction {
                    meta_data,
                    instruction: i32::from(InitializeMarketplace),
                    accounts,
                    args,
                    parsed,
                    inner_instructions,
                }
            }
            64 => {
                GalacticMarketplaceInstruction {
                    meta_data,
                    instruction: i32::from(CreateAccountWithSeed),
                    accounts,
                    args,
                    parsed,
                    inner_instructions,
                }
            }
            74 => {
                GalacticMarketplaceInstruction {
                    meta_data,
                    instruction: i32::from(UnknownTransaction),
                    accounts,
                    args,
                    parsed,
                    inner_instructions,
                }
            }
            85 => {
                GalacticMarketplaceInstruction {
                    meta_data,
                    instruction: i32::from(ProcessCancel),
                    accounts,
                    args,
                    parsed,
                    inner_instructions,
                }
            }
            112 => {
                // ProcessExchange
                accounts = match compiled_instruction.accounts.len() {
                    // 15 => {
                    //     map_account_names(transaction.message.clone().unwrap().account_keys, compiled_instruction.accounts.clone(), &PROCESS_EXCHANGE_ACCOUNTS_15)
                    // }
                    // 18 => {
                    //     map_account_names(transaction.message.clone().unwrap().account_keys, compiled_instruction.accounts.clone(), &PROCESS_EXCHANGE_ACCOUNTS_18)
                    // }

                    19 => {
                        match exchange_args.len() {
                            48 => {
                                info!("ProcessExchangeArgsWithPubkey 19");
                                let ProcessExchangeArgsWithPubkey { purchase_quantity, expected_price, seller } = ProcessExchangeArgsWithPubkey::try_from_slice(exchange_args)?;
                                {
                                    args.push(Arg { name: "PurchaseQuantity".to_string(), r#type: "u64".to_string(), value: purchase_quantity.to_string() });
                                    args.push(Arg { name: "ExpectedPrice".to_string(), r#type: "u64".to_string(), value: expected_price.to_string() });
                                    args.push(Arg { name: "Seller".to_string(), r#type: "String".to_string(), value: seller.to_string() });
                                }
                                map_account_names(transaction.message.clone().unwrap().account_keys, compiled_instruction.accounts.clone(), &PROCESS_EXCHANGE_ACCOUNTS_19)
                            }
                            _ => {
                                return Err(anyhow!("No 19 exchange_args len ProcessExchange for instruction: len={}", exchange_args.len()));
                            }
                        }
                    }
                    32 => {
                        match exchange_args.len() {
                            48 => {
                                info!("ProcessExchangeArgsWithPubkey 32");
                                //With ref?
                                let ProcessExchangeArgsWithPubkey { purchase_quantity, expected_price, seller } = ProcessExchangeArgsWithPubkey::try_from_slice(exchange_args)?;
                                {
                                    args.push(Arg { name: "PurchaseQuantity".to_string(), r#type: "u64".to_string(), value: purchase_quantity.to_string() });
                                    args.push(Arg { name: "ExpectedPrice".to_string(), r#type: "u64".to_string(), value: expected_price.to_string() });
                                    args.push(Arg { name: "Seller".to_string(), r#type: "String".to_string(), value: seller.to_string() });
                                }


                                let account_keys = append_extra_accounts(transaction);
                                info!("{:?}", transaction.message.clone().unwrap().account_keys.len());
                                info!("{:?}", transaction.message.clone().unwrap().address_table_lookups);
                                match transaction.message.clone().unwrap().account_keys.len() {
                                    30 => {
                                        map_account_names(account_keys, compiled_instruction.accounts.clone(), &PROCESS_EXCHANGE_ACCOUNTS_28)
                                    }
                                    _ => {
                                        map_account_names(account_keys, compiled_instruction.accounts.clone(), &PROCESS_EXCHANGE_ACCOUNTS_32)
                                    }
                                }
                            }
                            _ => {
                                return Err(anyhow!("No 32 exchange_args len ProcessExchange for instruction: len={}", exchange_args.len()));
                            }
                        }
                    }
                    _ => return Err(anyhow!("No exchange_args len for compiled_instruction.accounts: len={}", compiled_instruction.accounts.len()))
                };
                info!("Done here");
                if (accounts.len() == 0) {}


                //INSTRUCTIONS
                inner_instructions = map_inner_instruction(transaction, instruction_idx, meta.clone());

                //let mut seller = "".to_string();
                let mut taker = "".to_string();
                let mut maker = "".to_string();
                let mut seller = "".to_string();
                let mut currency = "".to_string();
                let mut asset = "".to_string();
                let mut side = "NONE".to_string();

                match inner_instructions.len() {
                    0 => {
                        match compiled_instruction.accounts.len() {
                            19 | 32 => {
                                taker = accounts.clone().into_iter().find(|acc| acc.name == "OrderTaker".to_string()).unwrap().address;
                                maker = accounts.clone().into_iter().find(|acc| acc.name == "OrderInitializer".to_string()).unwrap().address;
                                seller = args.clone().into_iter().find(|arg| arg.name == "Seller".to_string()).unwrap().value;
                                currency = accounts.clone().into_iter().find(|acc| acc.name == "CurrencyMint".to_string()).unwrap().address;
                                asset = accounts.clone().into_iter().find(|acc| acc.name == "AssetMint".to_string()).unwrap().address;
                                side = match args.clone().into_iter().find(|arg| arg.name == "Seller".to_string()).unwrap().value == accounts.clone().into_iter().find(|acc| acc.name == "OrderTaker".to_string()).unwrap().address
                                {
                                    true => { "BUY".to_string() }
                                    false => { "SELL".to_string() }
                                }
                            }

                            _ => return Err(anyhow!("No exchange_args len for compiled_instruction.accounts: while no inner instructions len={}", compiled_instruction.accounts.len()))
                        }
                    }
                    _ => {
                        match inner_instructions[0].clone().program {
                            None => {
                                return Err(anyhow!("Error mapping side!"));
                            }
                            Some(Program::TokenTransferChecked(inst_0)) => {
                                match inner_instructions[1].clone().program {
                                    None => { return Err(anyhow!("Error mapping side!")); }
                                    Some(Program::TokenTransferChecked(inst_1)) => {
                                        match inst_0.mint == inst_1.mint {
                                            true => {
                                                side = "SELL".to_string();
                                                currency = inst_0.mint;

                                                taker = match inner_instructions[1].clone().program {
                                                    None => { "".to_string() }
                                                    Some(Program::TokenTransferChecked(inst_1)) => {
                                                        inst_1.authority
                                                    }
                                                };
                                                maker = accounts[5].clone().address;
                                                asset = match inner_instructions[2].clone().program {
                                                    None => { "".to_string() }
                                                    Some(Program::TokenTransferChecked(inst_2)) => {
                                                        inst_2.mint
                                                    }
                                                };
                                                seller = maker.clone();
                                            }
                                            false => {
                                                side = "BUY".to_string();
                                                currency = inst_0.mint;
                                                asset = inst_1.mint;
                                                taker = match inner_instructions[1].clone().program {
                                                    None => { "".to_string() }
                                                    Some(Program::TokenTransferChecked(inst_1)) => {
                                                        inst_1.authority
                                                    }
                                                };

                                                maker = accounts[5].clone().address;
                                                seller = taker.clone();
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }


                // ADD EXTRAS
                let quantity = args.clone().into_iter().find(|a| ((a.name == "OriginationQty") || (a.name == "PurchaseQuantity"))).unwrap().value.parse::<f32>().unwrap();

                let mut price_no_decimals = 0.0;
                info!("{:?}", args);

                match args.clone().into_iter().find(|a| ((a.name == "Price") || (a.name == "ExpectedPrice"))) {
                    None => {
                        price_no_decimals = calc_price(inner_instructions.clone()).unwrap() / quantity;
                    }
                    Some(price) => {
                        price_no_decimals = price.value.parse::<f32>().unwrap();

                        if price_no_decimals == 0.0 {
                            price_no_decimals = calc_price(inner_instructions.clone()).unwrap() / quantity;
                        }
                    }
                };

                let price_decimals = get_currency_decimals(accounts.iter().find(|a| (&a.name == "CurrencyMint") || (&a.name == "ReceiveMint")).unwrap().clone().address);
                let fee =
                    match inner_instructions.len() {
                        3 | 4 => {
                            match inner_instructions[0].program.clone().unwrap() {
                                Program::TokenTransferChecked(transfer) => {
                                    transfer.token_amount.unwrap().ui_amount_string.parse::<f64>().unwrap()
                                }
                            }
                        }

                        0 => meta.clone().post_token_balances.into_iter().find(|t| t.owner == "feesQYAaH3wjGUUQYD959mmi5pY8HSz3F5C3SVc1fp3").unwrap().ui_token_amount.unwrap().ui_amount
                            - meta.clone().pre_token_balances.into_iter().find(|t| t.owner == "feesQYAaH3wjGUUQYD959mmi5pY8HSz3F5C3SVc1fp3").unwrap().ui_token_amount.unwrap().ui_amount,
                        _ => { return Err(anyhow!("No match on inner instruction length= {}!", inner_instructions.len())); }
                    };
                info!("fee={}", fee);

                let fee_decimals = meta.post_token_balances.into_iter().find(|t|
                    (t.owner == "feesQYAaH3wjGUUQYD959mmi5pY8HSz3F5C3SVc1fp3") || (t.owner == "MRKT9mCmNU2R4KnZt9BV5uh9MESj7Phxws4AR7fUhRc")
                ).unwrap().ui_token_amount.unwrap().decimals;


                // 1. Price
                parsed.push(Arg {
                    name: "price".to_string(),
                    r#type: "f32".to_string(),
                    value: (price_no_decimals * 10.0f32.powf(-(price_decimals))).to_string(),
                });
                // 2. Size
                parsed.push(Arg {
                    name: "size".to_string(),
                    r#type: "f32".to_string(),
                    value: (quantity).to_string(),
                });
                // 3. Volume
                parsed.push(Arg {
                    name: "volume".to_string(),
                    r#type: "f32".to_string(),
                    value: (quantity * price_no_decimals * 10.0f32.powf(-(price_decimals))).to_string(),
                });
                // 4. Side
                parsed.push(Arg {
                    name: "side".to_string(),
                    r#type: "String".to_string(),
                    value: side,
                });
                // 5. Fee
                parsed.push(Arg {
                    name: "fee".to_string(),
                    r#type: "String".to_string(),
                    value: format!("{:.1$}", fee, fee_decimals as usize),
                });
                // 6. buyer
                parsed.push(Arg {
                    name: "taker".to_string(),
                    r#type: "String".to_string(),
                    value: taker,
                });
                // 7. seller
                parsed.push(Arg {
                    name: "maker".to_string(),
                    r#type: "String".to_string(),
                    value: maker,
                });
                // 8. currency
                parsed.push(Arg {
                    name: "currency".to_string(),
                    r#type: "String".to_string(),
                    value: currency,
                });
                // 9. asset
                parsed.push(Arg {
                    name: "asset".to_string(),
                    r#type: "String".to_string(),
                    value: asset,
                });
                // 10. asset
                parsed.push(Arg {
                    name: "seller".to_string(),
                    r#type: "String".to_string(),
                    value: seller,
                });

                GalacticMarketplaceInstruction {
                    meta_data,
                    instruction: i32::from(ProcessExchange),
                    accounts,
                    args,
                    parsed,
                    inner_instructions,
                }
            }

            189 => {
                GalacticMarketplaceInstruction {
                    meta_data,
                    instruction: i32::from(DeregisterCurrency),
                    accounts,
                    args,
                    parsed,
                    inner_instructions,
                }
            }
            179 => {
                GalacticMarketplaceInstruction {
                    meta_data,
                    instruction: i32::from(UpdateCurrencyRoyalty),
                    accounts,
                    args,
                    parsed,
                    inner_instructions,
                }
            }
            221 => {
                GalacticMarketplaceInstruction {
                    meta_data,
                    instruction: i32::from(InitializeOpenOrdersCounter),
                    accounts,
                    args,
                    parsed,
                    inner_instructions,
                }
            }
            233 => {
                GalacticMarketplaceInstruction {
                    meta_data,
                    instruction: i32::from(UnknownTransaction),
                    accounts,
                    args,
                    parsed,
                    inner_instructions,
                }
            }
            247 => {
                GalacticMarketplaceInstruction {
                    meta_data,
                    instruction: i32::from(RegisterCurrency),
                    accounts,
                    args,
                    parsed,
                    inner_instructions,
                }
            }
            248 => {
                GalacticMarketplaceInstruction {
                    meta_data,
                    instruction: i32::from(UpdateAtlasRate),
                    accounts,
                    args,
                    parsed,
                    inner_instructions,
                }
            }
            _ => return Err(anyhow!("No tag for instruction: tag={}", tag))
        })
    }
}


fn map_account_names(account_list: Vec<Vec<u8>>, instruction_accounts: Vec<u8>, account_map: &[&str]) -> Vec<Account> {
    let mut accounts = vec![];

    info!("account_list={:?}", account_list.len());
    info!("instruction_accounts={:?}", instruction_accounts.len());
    info!("account_map={:?}", account_map.len());

    info!("account_list={:?}", account_list);
    info!("instruction_accounts={:?}", instruction_accounts);
    info!("account_map={:?}", account_map);

    for (account_name_idx, account_name) in account_map.into_iter().enumerate() {
        accounts.push(Account {
            name: account_name.to_string(),
            address: account_as_string(account_list.clone(), instruction_accounts.clone(), account_name_idx),
            is_mut: None,
            is_signer: Some(bs58::encode(&account_list[0].clone()).into_string() == account_as_string(account_list.clone(), instruction_accounts.clone(), account_name_idx)),
        })
    }
    accounts
}

fn map_inner_instruction(transaction: &Transaction, instruction_idx: usize, meta: TransactionStatusMeta) -> Vec<TokenProgram> {
    let mut inner_instructions = vec![];
    match meta.inner_instructions_none {
        false => {
            // We have inner instruction do it simple
            for inner_instruction in meta.inner_instructions.into_iter().find(|i| i.index == instruction_idx as u32).unwrap().instructions
            {
                //TEMP_FIX LEN instruction example at block: 253034209

                //Map only the ones with the Token-ProgramID
                let account_keys = append_extra_accounts(transaction);
                account_keys.clone().into_iter().for_each(|key| {
                    info!("key{:?}", bs58::encode(key).into_string())
                });
                if bs58::encode(&account_keys[inner_instruction.program_id_index as usize]).into_string().as_str() == TOKEN_PROGRAM {
                    if let Ok(parsed) = TokenProgram::unpack(inner_instruction, account_keys) {
                        inner_instructions.push(parsed)
                    }
                }
                info!("{:?}",inner_instructions);
            }
        }

        true => {
            // We dont have inner instruction
        }
    }
    inner_instructions
}

fn calc_price(inner_instructions: Vec<TokenProgram>) -> Result<f32, Error> {
    //Calc price manually
    let mut currency_amount_fee = 0;
    let mut currency_amount = 0;

    match inner_instructions[0].clone().program {
        Some(Program::TokenTransferChecked(inst_0)) => {
            currency_amount_fee = inst_0.token_amount.unwrap().amount;

            match inner_instructions[1].clone().program {
                Some(Program::TokenTransferChecked(inst_1)) => {
                    if inst_0.mint == inst_1.mint {
                        currency_amount = inst_1.token_amount.unwrap().amount
                    }
                }
                _ => {
                    return Err(anyhow!("no price"));
                }
            }
            match inner_instructions[2].clone().program {
                Some(Program::TokenTransferChecked(inst_2)) => {
                    if inst_0.mint.to_string() == inst_2.mint.to_string() {
                        currency_amount = inst_2.token_amount.unwrap().amount
                    }
                }
                _ => {
                    return Err(anyhow!("no price"));
                }
            }
        }
        _ => {
            return Err(anyhow!("no price"));
        }
    }


    info!("{:?}\n", currency_amount);

    info!("{:?}\n", currency_amount_fee);

    Ok((currency_amount + currency_amount_fee) as f32)
}


fn append_extra_accounts(transaction: &Transaction) -> Vec<Vec<u8>> {
    let mut account_keys = transaction.message.clone().unwrap().account_keys;

    if transaction.message.clone().unwrap().address_table_lookups.len() > 0 {
        let lookup = transaction.message.clone().unwrap().address_table_lookups[0].clone();
        let mut extra_accounts: Vec<Vec<u8>> = vec![];
        extra_accounts = lookup.readonly_indexes.into_iter().map(|index| bs58::decode(LOOKUP_TABLE[index as usize].to_string()).into_vec().unwrap()).collect();
        account_keys.append(&mut extra_accounts);
    }
    account_keys
}


