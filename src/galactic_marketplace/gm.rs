use anyhow::anyhow;
use borsh::BorshDeserialize;
use substreams::errors::Error;

use crate::galactic_marketplace::gm_accounts::{PROCESS_CANCEL_ACCOUNTS, PROCESS_EXCHANGE_ACCOUNTS_19, PROCESS_INITIALIZE_ACCOUNTS};
use crate::galactic_marketplace::gm_args::{ProcessExchangeArgsWithPubkey, ProcessInitializeSellArgs};
use crate::helper::base2string::account_as_string;
use crate::pb::sol::transaction::instruction::v1::{Account, Arg};

#[derive(Clone, Debug, PartialEq)]
pub enum MarketplaceInstruction {
    UnknownTransaction,
    ProcessExchange {
        accounts: Vec<Account>,
        args: Vec<Arg>,
        additional: Vec<Arg>,
    },
    ProcessInitializeSell {
        accounts: Vec<Account>,
        args: Vec<Arg>,
    },
    InitializeMarketplace,
    ProcessCancel {
        accounts: Vec<Account>,
    },
    DeregisterCurrency,
    ProcessInitializeBuy {
        accounts: Vec<Account>,
        args: Vec<Arg>,
    },
    UpdateCurrencyRoyalty,
    InitializeOpenOrdersCounter,
    RegisterCurrency,
    UpdateAtlasRate,
}


impl MarketplaceInstruction {
    pub fn unpack(input: &[u8], account_list: Vec<Vec<u8>>, instruction_accounts: Vec<u8>) -> Result<Self, Error> {
        let (&tag, rest) = input.split_first().ok_or(anyhow!("Unable to split instruction data"))?;
        let (_dump, exchange_args) = rest.split_at(7);

        let mut args = vec![];

        Ok(match tag {
            9 => {
                Self::UnknownTransaction
            }
            12 => {
                Self::UnknownTransaction
            }
            43 | 129 => {
                match exchange_args.len() {
                    16 => {
                        match ProcessInitializeSellArgs::try_from_slice(exchange_args.clone())? {
                            ProcessInitializeSellArgs { price, origination_qty } => {
                                args.push(Arg { name: "price".to_string(), r#type: "u64".to_string(), value: price.to_string() });
                                args.push(Arg { name: "origination_qty".to_string(), r#type: "u64".to_string(), value: origination_qty.to_string() });
                            }
                        }
                    }
                    _ => {
                        return Err(anyhow!("No exchange_args for ProcessInitializeSellArgs len for instruction: len={}", exchange_args.len()));
                    }
                }

                match tag {
                    43 => {
                        MarketplaceInstruction::ProcessInitializeSell {
                            accounts: map_account_names(account_list, instruction_accounts, &PROCESS_INITIALIZE_ACCOUNTS),
                            args,
                        }
                    }
                    129 => {
                        MarketplaceInstruction::ProcessInitializeBuy {
                            accounts: map_account_names(account_list, instruction_accounts, &PROCESS_INITIALIZE_ACCOUNTS),
                            args,
                        }
                    }
                    _ => {
                        return Err(anyhow!("No tag for instruction: tag={}", tag));
                    }
                }
            }
            47 => {
                Self::InitializeMarketplace
            }
            74 => {
                Self::UnknownTransaction
            }
            85 => {
                Self::ProcessCancel {
                    accounts: map_account_names(account_list, instruction_accounts, &PROCESS_CANCEL_ACCOUNTS),
                }
            }
            112 => {
                match exchange_args.len() {
                    48 => {
                        let ProcessExchangeArgsWithPubkey { purchase_quantity, expected_price, seller } = ProcessExchangeArgsWithPubkey::try_from_slice(exchange_args.clone())?;
                        {
                            args.push(Arg { name: "purchase_quantity".to_string(), r#type: "u64".to_string(), value: purchase_quantity.to_string() });
                            args.push(Arg { name: "expected_price".to_string(), r#type: "u64".to_string(), value: expected_price.to_string() });
                            args.push(Arg { name: "seller".to_string(), r#type: "String".to_string(), value: seller.to_string() });
                        }
                    }
                    _ => {
                        return Err(anyhow!("No exchange_args for ProcessExchangeArgsWithPubkey len for instruction: len={}", exchange_args.len()));
                    }
                }

                let mut additional = vec![];
                additional.push(Arg {
                    name: "price".to_string(),
                    r#type: "f64".to_string(),
                    value: "".to_string(),
                });

                MarketplaceInstruction::ProcessExchange {
                    accounts: map_account_names(account_list, instruction_accounts, &PROCESS_EXCHANGE_ACCOUNTS_19),
                    args,
                    additional,
                }
            }
            189 => {
                Self::DeregisterCurrency
            }
            179 => {
                Self::UpdateCurrencyRoyalty
            }
            221 => {
                Self::InitializeOpenOrdersCounter
            }
            233 => {
                Self::UnknownTransaction
            }
            247 => {
                Self::RegisterCurrency
            }
            248 => {
                Self::RegisterCurrency
            }
            _ => return Err(anyhow!("No tag for UNKNOWN instruction: tag={}", tag))
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




