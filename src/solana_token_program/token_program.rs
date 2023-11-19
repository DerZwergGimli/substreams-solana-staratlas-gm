use anyhow::anyhow;
use borsh::BorshDeserialize;
use log::info;
use substreams::errors::Error;
use substreams_solana::pb::sol::v1::CompiledInstruction;
use crate::helper::base2string::account_as_string;
use crate::pb::sol::token::program::v1::token_program::Program;
use crate::pb::sol::token::program::v1::{TokenProgram, TokenTransferChecked};
use crate::pb::sol::token::program::v1::token_transfer_checked::TokenAmount;
use crate::solana_token_program::token_program_accounts::TransferAmounts;

impl TokenProgram {
    pub fn unpack(instruction: CompiledInstruction, account_list: Vec<Vec<u8>>) -> Result<TokenProgram, Error> {
        let (&tag, rest) = instruction.data.split_first().ok_or(anyhow!("Unable to split instruction data"))?;

        info!("Accounts : {:?}", instruction.accounts);

        Ok(match tag {
            12 => {
                match rest.len() {
                    9 => {
                        match TransferAmounts::try_from_slice(rest) {
                            Ok(transfer_amounts) => {
                                TokenProgram {
                                    program: Some(Program::TokenTransferChecked(
                                        TokenTransferChecked {
                                            authority: account_as_string(account_list.clone(), instruction.clone().accounts, 3),
                                            destination: account_as_string(account_list.clone(), instruction.clone().accounts, 2),
                                            mint: account_as_string(account_list.clone(), instruction.clone().accounts, 1),
                                            source: account_as_string(account_list.clone(), instruction.clone().accounts, 0),
                                            token_amount: Some(
                                                TokenAmount {
                                                    amount: transfer_amounts.amount,
                                                    decimals: transfer_amounts.decimals as u32,
                                                    ui_amount: u64_to_float_with_decimals(transfer_amounts.amount, transfer_amounts.decimals) as f32,
                                                    ui_amount_string: u64_to_float_with_decimals(transfer_amounts.amount, transfer_amounts.decimals).to_string(),
                                                }
                                            ),
                                        }
                                    ))
                                }
                            }
                            Err(err) => {
                                return Err(anyhow!(err));
                            }
                        }
                    }
                    _ => {
                        return Err(anyhow!("SolProgram - tag={} length not found in rest={}", tag, rest.len()));
                    }
                }
            }

            _ => {
                return Err(anyhow!("SolProgram - No tag for instruction: tag={}", tag));
            }
        })
    }
}

fn u64_to_float_with_decimals(u64_value: u64, decimals: u8) -> f64 {
    let multiplier = 10u64.pow(decimals.into()) as f64;
    (u64_value as f64) / multiplier
}
