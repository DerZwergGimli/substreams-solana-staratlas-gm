use anyhow::anyhow;
use borsh::BorshDeserialize;
use substreams::errors::Error;
use substreams::log::info;
use substreams_solana::pb::sol::v1::{Block, CompiledInstruction, Transaction, TransactionStatusMeta};

use crate::galactic_marketplace::currencies::get_currency_decimals;
use crate::galactic_marketplace::gm_accounts::{PROCESS_EXCHANGE_ACCOUNTS_15, PROCESS_EXCHANGE_ACCOUNTS_19, PROCESS_EXCHANGE_ACCOUNTS_32, PROCESS_INITIALIZE_ACCOUNTS};
use crate::galactic_marketplace::gm_args::{ProcessExchangeArgNoPubkey, ProcessExchangeArgNoPubkeyAndPrice, ProcessExchangeArgsWithPubkey, ProcessInitializeSellArgs};
use crate::helper::base2string::account_as_string;
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
                        map_account_names(transaction.message.clone().unwrap().account_keys, compiled_instruction.accounts.clone(), &PROCESS_INITIALIZE_ACCOUNTS)
                    }
                    _ => return Err(anyhow!("No exchange_args len for compiled_instruction.accounts: len={}", compiled_instruction.accounts.len()))
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
                match exchange_args.len() {
                    8 => {
                        let ProcessExchangeArgNoPubkeyAndPrice { purchase_quantity } = ProcessExchangeArgNoPubkeyAndPrice::try_from_slice(exchange_args)?;
                        {
                            args.push(Arg { name: "PurchaseQuantity".to_string(), r#type: "u64".to_string(), value: purchase_quantity.to_string() });
                        }
                    }
                    14 => {
                        let ProcessInitializeSellArgs { price, origination_qty } = ProcessInitializeSellArgs::try_from_slice(exchange_args)?;
                        {
                            args.push(Arg { name: "Price".to_string(), r#type: "u64".to_string(), value: price.to_string() });
                            args.push(Arg { name: "OriginationQty".to_string(), r#type: "u64".to_string(), value: origination_qty.to_string() });
                        }
                    }
                    16 => {
                        let ProcessExchangeArgNoPubkey { expected_price, purchase_quantity } = ProcessExchangeArgNoPubkey::try_from_slice(exchange_args)?;
                        {
                            args.push(Arg { name: "Price".to_string(), r#type: "u64".to_string(), value: expected_price.to_string() });
                            args.push(Arg { name: "OriginationQty".to_string(), r#type: "u64".to_string(), value: purchase_quantity.to_string() });
                        }
                    }

                    48 => {
                        let ProcessExchangeArgsWithPubkey { purchase_quantity, expected_price, seller } = ProcessExchangeArgsWithPubkey::try_from_slice(exchange_args)?;
                        {
                            args.push(Arg { name: "PurchaseQuantity".to_string(), r#type: "u64".to_string(), value: purchase_quantity.to_string() });
                            args.push(Arg { name: "ExpectedPrice".to_string(), r#type: "u64".to_string(), value: expected_price.to_string() });
                            args.push(Arg { name: "Seller".to_string(), r#type: "String".to_string(), value: seller.to_string() });
                        }
                    }
                    _ => {
                        return Err(anyhow!("No exchange_args len ProcessExchange for instruction: len={}", exchange_args.len()));
                    }
                }

                //MAPPING
                accounts = match compiled_instruction.accounts.len() {
                    15 => {
                        map_account_names(transaction.message.clone().unwrap().account_keys, compiled_instruction.accounts.clone(), &PROCESS_EXCHANGE_ACCOUNTS_15)
                    }
                    19 => {
                        map_account_names(transaction.message.clone().unwrap().account_keys, compiled_instruction.accounts.clone(), &PROCESS_EXCHANGE_ACCOUNTS_19)
                    }
                    32 => {
                        map_account_names(transaction.message.clone().unwrap().account_keys, compiled_instruction.accounts.clone(), &PROCESS_EXCHANGE_ACCOUNTS_32)
                    }
                    _ => return Err(anyhow!("No exchange_args len for compiled_instruction.accounts: len={}", compiled_instruction.accounts.len()))
                };

                //INSTRUCTIONS
                inner_instructions = map_inner_instruction(transaction, instruction_idx, meta.clone());


                let mut seller = "".to_string();
                let mut side = "NONE".to_string();
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
                                        seller = match inner_instructions[2].clone().program {
                                            None => { "".to_string() }
                                            Some(Program::TokenTransferChecked(inst_2)) => {
                                                inst_2.authority
                                            }
                                        };
                                    }
                                    false => {
                                        side = "BUY".to_string();
                                        seller = inst_1.authority;
                                    }
                                }
                            }
                        }
                    }
                }
                // return Err(anyhow!("Error mapping side!"));


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
                        3 => {
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
                    name: "buyer".to_string(),
                    r#type: "String".to_string(),
                    value: accounts.clone().into_iter().find(|a| a.name == "OrderInitializer").unwrap().address,
                });
                // 7. seller
                parsed.push(Arg {
                    name: "seller".to_string(),
                    r#type: "String".to_string(),
                    value: seller,
                });
                // 8. currency
                parsed.push(Arg {
                    name: "currency".to_string(),
                    r#type: "String".to_string(),
                    value: accounts.clone().into_iter().find(|a| (a.name == "ReceiveMint") || (a.name == "CurrencyMint")).unwrap().address,
                });
                // 9. asset
                parsed.push(Arg {
                    name: "asset".to_string(),
                    r#type: "String".to_string(),
                    value: accounts.clone().into_iter().find(|a| (a.name == "DepositMint") || (a.name == "AssetMint")).unwrap().address,
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
    for (account_name_idx, account_name) in account_map.into_iter().enumerate() {
        accounts.push(Account {
            name: account_name.to_string(),
            address: account_as_string(account_list.clone(), instruction_accounts.clone(), account_name_idx),
            is_mut: None,
            is_signer: None,
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
                //Map only the ones with the Token-ProgramID
                if bs58::encode(&transaction.message.clone().unwrap().account_keys[inner_instruction.program_id_index as usize]).into_string().as_str() == TOKEN_PROGRAM {
                    if let Ok(parsed) = TokenProgram::unpack(inner_instruction, transaction.message.clone().unwrap().account_keys) {
                        inner_instructions.push(parsed)
                    }
                }
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




