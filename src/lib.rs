use std::io::{Error as ioError, Read};
use std::error::Error;
use transaction::{Amount, Inputs, Outputs, Transaction, Txid};
use sha2::{Sha256, Digest};
mod transaction;


fn read_compact_size(transaction_bytes: &mut &[u8]) -> Result<u64, ioError>  {
    let mut compact_size = [0_u8; 1];
    transaction_bytes.read(&mut compact_size)?;

    match compact_size[0] {
        0..=252 => Ok(compact_size[0] as u64),
        253 => {
            let mut buffer = [0; 2];
            transaction_bytes.read(&mut buffer)?;
            Ok(u16::from_le_bytes(buffer) as u64)
        }
        254 => {
            let mut buffer = [0; 4];
            transaction_bytes.read(&mut buffer)?;
            Ok(u32::from_le_bytes(buffer) as u64)
        }
        255 => {
            let mut buffer = [0; 8];
            transaction_bytes.read(&mut buffer)?;
            Ok(u64::from_le_bytes(buffer))
        }
    }
    
}
fn read_u32(transaction_bytes: &mut &[u8]) -> Result<u32, ioError> {
    let mut buffer = [0; 4];
    transaction_bytes.read(&mut buffer)?;
    Ok(u32::from_le_bytes(buffer))
}
fn read_amount(transaction_bytes: &mut &[u8]) -> Result<Amount, ioError> {
    let mut buffer = [0; 8];
    transaction_bytes.read(&mut buffer)?;
    Ok(Amount::from_sat(u64::from_le_bytes(buffer)))
}


fn read_txid(transaction_bytes: &mut &[u8]) -> Result<Txid, ioError> {
    let mut buffer = [0; 32];
    transaction_bytes.read(&mut buffer)?;
    Ok(Txid::from_bytes(buffer))
}
fn read_script(transaction_bytes: &mut &[u8]) -> Result<String, ioError> {
    let script_length = read_compact_size(transaction_bytes)? as usize;
    let mut buffer = vec![0; script_length];
    transaction_bytes.read(&mut buffer)?;
    Ok(hex::encode(buffer)) 
}
fn hash_raw_transaction(raw_transaction:  &[u8]) -> Txid {
    let mut hasher = Sha256::new();
    hasher.update(&raw_transaction);
    let result1 = hasher.finalize();

    let mut hasher = Sha256::new();
    hasher.update(&result1);
    let result = hasher.finalize();
    Txid::from_bytes(result.into())
}
pub fn decode(transaction_hex: String) -> Result<String, Box<dyn Error>> {
    let transaction_bytes = hex::decode(transaction_hex).map_err(|e| format!("Hex decode error : {} ", e))?;
    let mut bytes_slice = transaction_bytes.as_slice();
    let version = read_u32(&mut bytes_slice)?;
    let input_count = read_compact_size(&mut bytes_slice)?;
    let mut inputs = vec![];
    for _ in 0..input_count {
        let txid = read_txid(&mut bytes_slice)?;
        let output_index = read_u32(&mut bytes_slice)?;
        let script_sig = read_script(&mut bytes_slice)?;
        let sequence = read_u32(&mut bytes_slice)?;
        inputs.push(Inputs {
            txid,
            output_index,
            script_sig,
            sequence,
        });
    }
    let output_count = read_compact_size(&mut bytes_slice)?;
    let mut outputs = vec![];
    for _ in 0..output_count {
        let amount = read_amount(&mut bytes_slice)?;
        let script_pubkey = read_script(&mut bytes_slice)?;
        outputs.push(Outputs { 
            amount,
            script_pubkey 
        });
    }
    let lock_time = read_u32(&mut bytes_slice)?;
    let transaction_id =hash_raw_transaction(&transaction_bytes); // transaction_bytes
    let transaction = Transaction {
        transaction_id,
        version,
        inputs,
        outputs,
        lock_time,
        
    };
    // let transaction_json = serde_json::to_string_pretty(&transaction)?;
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