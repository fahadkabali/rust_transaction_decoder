// use std::io::BufRead
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

#[derive(Debug, Serialize)]
pub struct Transaction {
    pub transaction_id: Txid,
    pub version: u32,
    pub inputs: Vec<Inputs>,
    pub outputs: Vec<Outputs>,
    pub lock_time: u32,
}
#[derive(Debug, Deserialize)]
pub struct Txid([u8; 32]);
impl Txid {
    pub fn from_bytes(bytes: [u8; 32]) -> Txid {
        Txid(bytes)
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
pub struct Inputs {
    pub txid: Txid,
    pub output_index: u32,
    pub script_sig: String,
    pub sequence: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Outputs {
    #[serde(serialize_with = "as_btc")]
    pub amount: Amount,
    pub script_pubkey: String,
}
fn as_btc<S: Serializer, T: BitcoinValue >(t: &T, s:S) -> Result<S::Ok, S::Error> {
    let btc = t.to_btc();
    s.serialize_f64(btc)
}
pub trait Decodable : Sized{
    fn consensus_decode <R: BufRead + ?Sized>(r: &mut R) -> Result<Self, Error>;
    
}