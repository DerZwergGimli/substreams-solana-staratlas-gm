use log::info;
use substreams::errors::Error;
use substreams::log;
use substreams::store::{StoreAdd, StoreAddFloat64};


use db::db::add_process_exchange;
use substreams_database_change::pb::database::{DatabaseChanges};
use substreams::store::StoreNew;
use substreams_solana::pb::sf::solana::r#type::v1::Block;
use crate::galactic_marketplace::GM_PROGRAM;
use crate::pb::sa::gm::market::v1::{GalacticMarketplaceInstruction, GalacticMarketplaceInstructions};

mod pb;

mod helper;
mod galactic_marketplace;
mod db;
mod solana_token_program;
mod lookup;

#[substreams::handlers::map]
fn map_market_instructions(blk: Block) -> Result<GalacticMarketplaceInstructions, Error> {
    log::info!("map_market_instructions");
    let mut galactic_marketplace_instructions = vec![];
    process_blocks_for_instructions(blk, &mut galactic_marketplace_instructions).unwrap();
    Ok(GalacticMarketplaceInstructions {
        galactic_marketplace_instructions
    })
}

#[substreams::handlers::store]
pub fn store_order_book_updates(
    galactic_marketplace_instructions: GalacticMarketplaceInstructions,
    store: StoreAddFloat64) {
    for instruction in galactic_marketplace_instructions.galactic_marketplace_instructions {
        match instruction.instruction {
            4 => {
                //PROCESS_INITIALIZE_BUY
                store.add(
                    instruction.meta_data.unwrap().block,
                    instruction.parsed.into_iter().find(|a| a.name == "").unwrap().value,
                    5.0)
            }
            5 => {
                //PROCESS_INITIALIZE_SELLs
            }
            3 => {
                //PROCESS_EXCHANGE
            }
            2 => {
                //PROCESS_CANCEL
            }
            _ => {}
        }
    }
}

#[substreams::handlers::map]
pub fn db_out(
    galactic_marketplace_instructions: GalacticMarketplaceInstructions,
) -> Result<DatabaseChanges, Error> {
    let mut tables = substreams_database_change::tables::Tables::new();
    add_process_exchange(&mut tables, galactic_marketplace_instructions);
    Ok(tables.to_database_changes())
}

fn process_blocks_for_instructions(block: Block, instructions: &mut Vec<GalacticMarketplaceInstruction>) -> Result<(), Error> {
    for trx in block.clone().transactions {
        if let Some(meta) = trx.clone().meta {
            if meta.err.is_some() {
                continue;
            }

            if let Some(transaction) = trx.clone().transaction {
                if let Some(message) = transaction.clone().message {
                    for (instruction_idx, instruction) in message.instructions.clone().into_iter().enumerate() {
                        let program_id = &message.account_keys[instruction.program_id_index as usize];
                        let signature = bs58::encode(transaction.signatures[0].as_slice()).into_string();

                        if bs58::encode(program_id).into_string().as_str() != GM_PROGRAM {
                            continue;
                        }


                        info!("signature = {:?}", signature);
                        match GalacticMarketplaceInstruction::unpack(
                            block.clone(),
                            &transaction,
                            &instruction,
                            instruction_idx,
                            meta.clone()) {
                            Ok(data) => {
                                instructions.push(data)
                            }
                            Err(err) => {
                                log::info!("signature=\n{}\n", signature);
                                log::info!("error=\n{}\n", err);
                                return Err(err);
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(())
}



