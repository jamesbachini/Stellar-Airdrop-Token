use std::{
    env,
    fs::{self},
    io::Write,
    io::Read,
    str::FromStr,
};

use serde::{Deserialize, Serialize};

use merkle::MerkleTree;
use stellar_xdr::curr::{
    Int128Parts, Limits, ScAddress, ScMap, ScMapEntry, ScSymbol, ScVal, StringM, WriteXdr,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Receiver {
    pub address: String,
    pub amount: i128,
}

#[derive(Debug, Clone, Serialize)]
struct Proofs {
    pub index: usize,
    pub receiver: Receiver,
    pub proofs: Vec<String>,
}

fn make_receiver(index: usize, address: &str, amount: i128) -> Result<ScVal, ()> {
    let i128parts = Int128Parts {
        hi: (amount >> 64) as i64,
        lo: amount as u64,
    };
    let entries = vec![
        ScMapEntry {
            key: ScVal::Symbol(ScSymbol(StringM::from_str("index")?)),
            val: ScVal::U32(index as u32),
        },
        ScMapEntry {
            key: ScVal::Symbol(ScSymbol(StringM::from_str("address")?)),
            val: ScVal::Address(ScAddress::from_str(address)?),
        },
        ScMapEntry {
            key: ScVal::Symbol(ScSymbol(StringM::from_str("amount")?)),
            val: ScVal::I128(i128parts),
        },
    ]
    .into_iter();
    let map = ScMap::sorted_from_entries(entries)?;
    Ok(ScVal::Map(Some(map)))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let receiver_filename = args
        .get(1)
        .ok_or_else(|| "Missing receivers filename argument".to_string())?
        .as_str();
    let proofs_filename = args
        .get(2)
        .ok_or_else(|| "Missing proofs filename argument".to_string())?
        .as_str();
    println!("receiver_filename {}",receiver_filename);
    let metadata = fs::metadata(receiver_filename)
        .map_err(|e| format!("Failed to get metadata: {}", e))?;
    if metadata.len() == 0 {
        return Err("Receiver JSON file is empty".into());
    }
    let mut file = fs::File::open(receiver_filename)
        .map_err(|e| format!("Failed to open file: {}", e))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(|e| format!("Failed to read file to string: {}", e))?;
    //println!("Read JSON content: {}", &contents[..std::cmp::min(contents.len(), 100)]);
    let receivers: Vec<Receiver> = serde_json::from_str(&contents)
        .map_err(|e| format!("Failed to parse JSON: {}", e))?; 
    let serialized_receivers: Vec<Vec<u8>> = receivers
        .iter().enumerate()
        .map(|(index,receiver)| {
                make_receiver(index, &receiver.address, receiver.amount).unwrap()
            })        
        .map(|val|  val.to_xdr(Limits::none()).unwrap())
        .collect();
    let tree = MerkleTree::new(serialized_receivers.clone());
    println!("root: {}", hex::encode(tree.root().unwrap()));
    let proofs: Vec<Proofs> = serialized_receivers
        .iter().enumerate()
        .map(|(index, data)| Proofs {
            index,
            receiver: receivers.get(index).unwrap().clone(),
            proofs: tree
                .get_proof(data.clone())
                .unwrap_or_default()
                .iter()
                .map(hex::encode)
                .collect(),
        })
        .collect();
    let proofs_content = serde_json::to_string_pretty(&proofs)
        .map_err(|e| format!("Failed to serialize proofs: {}", e))?;
    let mut proofs_file = fs::File::create(proofs_filename)?;
    proofs_file
        .write_all(proofs_content.as_bytes())
        .map_err(|e| format!("Failed to write proofs to file {}", e))?;
    Ok(())
}
