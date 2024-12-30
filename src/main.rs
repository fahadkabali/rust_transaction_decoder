use core::fmt;
use std::io::Read;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Transaction {
    version: u32,
    inputs: Vec<Inputs>,
    outputs: Vec<Outputs>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Inputs {
    txid: String,
    output_index: u32,
    script_sig: String,
    sequence: u32,
}
#[derive(Debug, Serialize, Deserialize)]
struct Outputs {
    amount: u64,
    script_pubkey: String,
}
fn read_compact_size(transaction_bytes: &mut &[u8]) -> u64 {
    let mut compact_size = [0_u8; 1];
    transaction_bytes.read(&mut compact_size).unwrap();

    match compact_size[0] {
        0..=252 => compact_size[0] as u64,
        253 => {
            let mut buffer = [0; 2];
            transaction_bytes.read(&mut buffer).unwrap();
            u16::from_le_bytes(buffer) as u64
        }
        254 => {
            let mut buffer = [0; 4];
            transaction_bytes.read(&mut buffer).unwrap();
            u32::from_le_bytes(buffer) as u64
        }
        255 => {
            let mut buffer = [0; 8];
            transaction_bytes.read(&mut buffer).unwrap();
            u64::from_le_bytes(buffer)
        }
    }
    // if compact_size[0] < 0xfd {
    //     return compact_size[0] as u64;
    // }
    // if compact_size[0] == 0xfd {
    //     let mut buffer = [0_u8; 2];
    //     transaction_bytes.read(&mut buffer).unwrap();
    //     u16::from_le_bytes(buffer) as u64
    // } else if compact_size[0] == 0xfe {
    //     let mut buffer = [0_u8; 4];
    //     transaction_bytes.read(&mut buffer).unwrap();
    //     u32::from_le_bytes(buffer) as u64
    // } else {
    //     let mut buffer = [0_u8; 8];
    //     transaction_bytes.read(&mut buffer).unwrap();
    //     u64::from_le_bytes(buffer)
    // }
}
fn read_u32(transaction_bytes: &mut &[u8]) -> u32 {
    let mut buffer = [0; 4];
    transaction_bytes.read(&mut buffer).unwrap();
    u32::from_le_bytes(buffer)
}
fn read_u64(transaction_bytes: &mut &[u8]) -> u64 {
    let mut buffer = [0; 8];
    transaction_bytes.read(&mut buffer).unwrap();
    u64::from_le_bytes(buffer)
}

// enum ScriptType {
//     P2PKH(String),
//     P2SH(String),
//     P2WPKH(String),
//     P2WSH(String),
// }
fn read_txid(transaction_bytes: &mut &[u8]) -> String {
    let mut buffer = [0; 32];
    transaction_bytes.read(&mut buffer).unwrap();
    buffer.reverse();
    hex::encode(buffer) //buffer
}
fn read_script(transaction_bytes: &mut &[u8]) -> String {
    let script_length = read_compact_size(transaction_bytes) as usize;
    let mut buffer = vec![0; script_length];
    transaction_bytes.read(&mut buffer).unwrap();
    hex::encode(buffer) //buffer
}
fn main() {
    let transaction_hex = "010000000242d5c1d6f7308bbe95c0f6e1301dd73a8da77d2155b0773bc297ac47f9cd7380010000006a4730440220771361aae55e84496b9e7b06e0a53dd122a1425f85840af7a52b20fa329816070220221dd92132e82ef9c133cb1a106b64893892a11acf2cfa1adb7698dcdc02f01b0121030077be25dc482e7f4abad60115416881fe4ef98af33c924cd8b20ca4e57e8bd5feffffff75c87cc5f3150eefc1c04c0246e7e0b370e64b17d6226c44b333a6f4ca14b49c000000006b483045022100e0d85fece671d367c8d442a96230954cdda4b9cf95e9edc763616d05d93e944302202330d520408d909575c5f6976cc405b3042673b601f4f2140b2e4d447e671c47012103c43afccd37aae7107f5a43f5b7b223d034e7583b77c8cd1084d86895a7341abffeffffff02ebb10f00000000001976a9144ef88a0b04e3ad6d1888da4be260d6735e0d308488ac508c1e000000000017a91476c0c8f2fc403c5edaea365f6a284317b9cdf7258700000000";
    let transaction_bytes = hex::decode(transaction_hex).unwrap();
    let mut bytes_slice = transaction_bytes.as_slice();
    let version = read_u32(&mut bytes_slice);
    let input_count = read_compact_size(&mut bytes_slice);
    let mut inputs = vec![];
    for _ in 0..input_count {
        let txid = read_txid(&mut bytes_slice);
        let output_index = read_u32(&mut bytes_slice);
        let script_sig = read_script(&mut bytes_slice);
        let sequence = read_u32(&mut bytes_slice);
        inputs.push(Inputs {
            txid,
            output_index,
            script_sig,
            sequence,
        });
    }
    let output_count = read_compact_size(&mut bytes_slice);
    let mut outputs = vec![];
    for _ in 0..output_count {
        let amount = read_u64(&mut bytes_slice);
        let script_pubkey = read_script(&mut bytes_slice);
        outputs.push(Outputs { 
            amount,
            script_pubkey 
        });
    }
    let transaction = Transaction {
        version,
        inputs,
        outputs,
    };
    let mut transaction_json = serde_json::to_string_pretty(&transaction).unwrap();
    println!("Transaction: {}", transaction_json);
}

#[cfg(test)]
mod tests {
    use super::read_compact_size;
    #[test]
    fn test_read_compact_size() {
        let mut bytes = [1_u8].as_slice();
        let count = read_compact_size(&mut bytes);
        assert_eq!(count, 1_u64);

        let mut bytes = [253_u8, 0, 1].as_slice();
        let count = read_compact_size(&mut bytes);
        assert_eq!(count, 256_u64);

        let mut bytes = [254_u8, 0, 0, 0, 1].as_slice();
        let count = read_compact_size(&mut bytes);
        assert_eq!(count, 256_u64.pow(3));

        let mut bytes = [255_u8, 0, 0, 0, 0, 0, 0, 0, 1].as_slice();
        let count = read_compact_size(&mut bytes);
        assert_eq!(count, 256_u64.pow(7));

        let hex = "fd204e";
        let decoded = hex::decode(hex).unwrap();
        let mut bytes = decoded.as_slice();
        let count = read_compact_size(&mut bytes);
        assert_eq!(count, 20_000_u64);
    }
}