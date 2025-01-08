use std::error::Error;
use self::transaction::{Decodable, Transaction};
mod transaction;



pub fn decode(transaction_hex: String) -> Result<String, Box<dyn Error>> {
    let transaction_bytes = hex::decode(transaction_hex).map_err(|e| format!("Hex decode error : {} ", e))?;
    let transaction = Transaction::consensus_decode(&mut transaction_bytes.as_slice())?;
    Ok(serde_json::to_string_pretty(&transaction)?)
}


// #[cfg(test)]
// mod tests {
//     use super::read_compact_size;
//     use super::error::Error;
//     #[test]
//     fn test_read_compact_size() -> Result<(), Box<dyn Error>> {
//         let mut bytes = [1_u8].as_slice();
//         let count = read_compact_size(&mut bytes)?;
//         assert_eq!(count, 1_u64);

//         let mut bytes = [253_u8, 0, 1].as_slice();
//         let count = read_compact_size(&mut bytes)?;
//         assert_eq!(count, 256_u64);

//         let mut bytes = [254_u8, 0, 0, 0, 1].as_slice();
//         let count = read_compact_size(&mut bytes)?;
//         assert_eq!(count, 256_u64.pow(3));

//         let mut bytes = [255_u8, 0, 0, 0, 0, 0, 0, 0, 1].as_slice();
//         let count = read_compact_size(&mut bytes)?;
//         assert_eq!(count, 256_u64.pow(7));

//         let hex = "fd204e";
//         let decoded = hex::decode(hex)?;
//         let mut bytes = decoded.as_slice();
//         let count = read_compact_size(&mut bytes)?;
//         assert_eq!(count, 20_000_u64);

//         Ok(())
//     }
// }