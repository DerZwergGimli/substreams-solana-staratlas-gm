pub fn get_currency_decimals(mint: String) -> f32 {
    match mint.as_str() {
        "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v" => { 8.0 }
        "ATLASXmbPQxBUYbxPsV97usA3fPQYEqzQBUHgiFCUsXx" => { 8.0 }
        _ => { 0.0 }
    }
}