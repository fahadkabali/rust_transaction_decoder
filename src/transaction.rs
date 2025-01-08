use std::io::BufRead
// use std::io::{Error as ioError, BufRead}
use serde::{Serialize, Deserialize, Serializer};
use std::fmt;


#[derive(Debug)]
pub enum Error{
    Io(std::io::Error)
}
impl fmt::Display for Error{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Io(e) => write!(f, "IO Error: {}", e)
        }
    }
}

impl std::error::Error for Error {}

#[derive(Debug)]
pub struct Transaction {
    // pub transaction_id: Txid,
    pub version: u32,
    pub inputs: Vec<TxIn>,
    pub outputs: Vec<TxOut>,
    pub lock_time: u32,
}

impl Transaction{
    pub fn txid(&self) -> Txid{
        // todo: implement this
        let txid_data = vec![0;32];
        Txid::new(txid_data)
        }
}

impl Serialize for Transaction {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error{
        let mut tx = serializer.serialize_struct("Transaction", 5)?;
        tx.serialize_field("transaction_id", &self.txid())?;
        tx.serialize_field("version", &self.version)?;
        tx.serialize_field("inputs", &self.inputs)?;
        tx.serialize_field("outputs", &self.outputs)?;
        tx.serialize_field("lock_time", &self.lock_time)?;
        tx.end()
    }

}
#[derive(Debug, Deserialize)]
pub struct Txid([u8; 32]);
impl Txid {
    pub fn from_hash(bytes: [u8; 32]) -> Txid {
        Txid(bytes)
    }
    fn from_raw_transaction(tx: Vec<u8>) -> Txid{
        let mut hasher = Sha256::new();
        hasher.update(&raw_transaction);
        let result1 = hasher.finalize();
    
        let mut hasher = Sha256::new();
        hasher.update(&result1);
        let result = hasher.finalize();
        Txid::from_bytes(result.into())
    }
}
impl Serialize for Txid {
    fn serialize<S : Serializer>(&self, s: S) -> Result<S::Ok, S::Error>{
      let mut bytes = self.0.clone();
      bytes.reverse();
      s.serialize_str(&hex::encode(bytes))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Amount(u64);

impl Amount {
    pub fn from_sat(satoshi: u64) -> Amount {
        Amount(satoshi)
    }
}


trait BitcoinValue {
    fn to_btc(&self) -> f64;
}
impl BitcoinValue for Amount {
    fn to_btc(&self) -> f64 {
        self.0 as f64 / 100_000_000.0
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TxIn {
    pub txid: Txid,
    pub output_index: u32,
    pub script_sig: String,
    pub sequence: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TxOut {
    #[serde(serialize_with = "as_btc")]
    pub amount: Amount,
    pub script_pubkey: String,
}
fn as_btc<S: Serializer, T: BitcoinValue >(t: &T, s:S) -> Result<S::Ok, S::Error> {
    let btc = t.to_btc();
    s.serialize_f64(btc)
}

#[derive(Debug, Serialize)]
pub struct CompactSize(pub u64);

pub trait Decodable : Sized{
    fn consensus_decode <R: BufRead + ?Sized>(r: &mut R) -> Result<Self, Error>;
    
}
impl Decodable for u16{
    fn consensus_decode <R: BufRead + ?Sized>(r: &mut R) -> Result<Self, Error> {
        let mut buffer = [0;2];
        r.read_exact(&mut buffer).map_err(Error::Io)?;
        Ok(u16::from_le_bytes(buffer))
    }
}


impl Decodable for u32{
    fn consensus_decode <R: BufRead + ?Sized>(r: &mut R) -> Result<Self, Error> {
        let mut buffer = [0;4];
        r.read_exact(&mut buffer).map_err(Error::Io)?;
        Ok(u32::from_le_bytes(buffer))
    }
}

impl Decodable for u64{
    fn consensus_decode <R: BufRead + ?Sized>(r: &mut R) -> Result<Self, Error>{
        let mut buffer = [0;8];
        r.read_exact(&mut buffer).map_err(Error::Io)?;
        Ok(u64::from_le_bytes(buffer))
    }
}

impl Decodable for CompactSize{
    fn consensus_decode <R: BufRead + ?Sized>(r: &mut R) -> Result<Self, Error>{
        let n = u8::consensus_decode(r)?;
        match n {
            0xFF => {
                let x = u64::consensus_decode(r)?;
                Ok(CompactSize(x))
            },
            0xFE => {
                let x = u32::consensus_decode(r)?;
                Ok(CompactSize(x as u64))
            },
            0xFD => {
                let x = u16::consensus_decode(r)?;
                Ok(CompactSize(x as u64))
            },
        }
    }
}impl Decodable for String{
    fn consensus_decode <R: BufRead + ?Sized>(r: &mut R) -> Result<Self, Error>{
        let len = CompactSize::consensus_decode(r)?.0;
        let mut buffer = vec![0; len as usize];
        r.read_exact(&mut buffer).map_err(Error::Io)?;
        Ok(hex::encode(buffer))
    }
}

impl Decodable for Vec<TxIn>{
    fn consensus_decode <R: BufRead + ?Sized>(r: &mut R) -> Result<Self, Error>{
        let count = CompactSize::consensus_decode(r)?.0;
        let mut inputs = Vec::with_capacity(count as usize);
        for _ in 0..count {
            inputs.push(TxIn::consensus_decode(r)?);
        }
    }
}

impl Decodable for TxIn{
    fn consensus_decode <R: BufRead + ?Sized>(r: &mut R) -> Result<Self, Error>{
        Ok(TxIn{
            txid: Txid::consensus_decode(r)?,
            output_index: u32::consensus_decode(r)?,
            script_sig: String::consensus_decode(r)?,
            sequence: u32::consensus_decode(r)?,
        })
    }
}

impl Decodable for Txid{
    fn consensus_decode <R: BufRead + ?Sized>(r: &mut R) -> Result<Self, Error>{
        let mut buffer = [0; 32];
        r.read_exact(&mut buffer).map_err(Error::Io)?;
        Ok(Txid(buffer))
    }
}

impl Decodable for TxOut{
    fn consensus_decode <R: BufRead + ?Sized>(r: &mut R) -> Result<Self, Error>{
        Ok(TxOut{

        })
    }
}