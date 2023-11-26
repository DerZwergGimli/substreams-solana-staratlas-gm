use substreams_database_change::tables::Tables;
use crate::pb::sa::gm::market::v1::{GalacticMarketplaceInstruction, GalacticMarketplaceInstructions};


pub fn add_process_exchange(tables: &mut Tables, galactic_marketplace_instruction: GalacticMarketplaceInstructions) {
    for (instruction_idx, instruction) in galactic_marketplace_instruction.galactic_marketplace_instructions.into_iter().enumerate() {
        match instruction.instruction {
            3 => {
                push_create_exchange(tables, instruction, instruction_idx)
            }
            _ => {}
        }
    }
}


// pub fn update_order_book(tables: &mut Tables, instruction: GalacticMarketplaceInstruction) {
//     for instruction in instruction {
//         match instruction.name.as_str() {
//             "ProcessExchange" => {
//                 push_create_exchange(tables, instruction)
//             }
//             _ => {}
//         }
//     }
// }

fn push_create_exchange(tables: &mut Tables, instruction: GalacticMarketplaceInstruction, instruction_idx: usize) {
    tables
        .create_row("sa_trades", format!("{}{}", instruction.meta_data.clone().unwrap().signature, instruction_idx))

        .set("timestamp", instruction.meta_data.clone().unwrap().timestamp)
        .set("block", instruction.meta_data.clone().unwrap().block)
        .set("signature", instruction.meta_data.clone().unwrap().signature)
        .set("maker", instruction.parsed.clone().into_iter().find(|p| p.name == "maker").unwrap().value)
        .set("taker", instruction.parsed.clone().into_iter().find(|p| p.name == "taker").unwrap().value)
        .set("seller", instruction.parsed.clone().into_iter().find(|p| p.name == "seller").unwrap().value)
        .set("currency", instruction.parsed.clone().into_iter().find(|p| p.name == "currency").unwrap().value)
        .set("asset", instruction.parsed.clone().into_iter().find(|p| p.name == "asset").unwrap().value)
        .set("price", instruction.parsed.clone().into_iter().find(|p| p.name == "price").unwrap().value)
        .set("size", instruction.parsed.clone().into_iter().find(|p| p.name == "size").unwrap().value)
        .set("volume", instruction.parsed.clone().into_iter().find(|p| p.name == "volume").unwrap().value)
        .set("side", instruction.parsed.clone().into_iter().find(|p| p.name == "side").unwrap().value)
        .set("fee", instruction.parsed.clone().into_iter().find(|p| p.name == "fee").unwrap().value)
    ;
}


// pub fn add_block_meta_to_tables(tables: &mut Tables, deltas: Deltas<DeltaProto<BlockMeta>>) {
//     use substreams::pb::substreams::store_delta::Operation;
//
//     for delta in deltas.into_iter() {
//         match delta.operation {
//             Operation::Create => push_create(
//                 tables,
//                 &delta.key,
//                 BlockTimestamp::from_key(&delta.key),
//                 delta.new_value,
//             ),
//             Operation::Update => push_update(tables, &delta.key, delta.new_value),
//             Operation::Delete => panic!("delete should not happen"),
//             x => panic!("unsupported opeation {:?}", x),
//         }
//     }
// }
//
//
// fn push_create(tables: &mut Tables, key: &str, timestamp: BlockTimestamp, value: BlockMeta) {
//     tables
//         .create_row("block_meta", key)
//         .set("at", timestamp)
//         .set("number", value.number)
//         .set("hash", Hex(value.hash))
//         .set("parent_hash", Hex(value.parent_hash))
//         .set("timestamp", value.timestamp.unwrap());
// }
//
// fn push_update(tables: &mut Tables, key: &str, value: BlockMeta) {
//     tables
//         .update_row("block_meta", key)
//         .set("number", value.number)
//         .set("hash", Hex(value.hash))
//         .set("parent_hash", Hex(value.parent_hash))
//         .set("timestamp", value.timestamp.unwrap());
// }