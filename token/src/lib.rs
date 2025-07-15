#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, BytesN, Env, String, Vec};
use stellar_fungible::{Base, FungibleToken};
use stellar_crypto::sha256::Sha256;
use stellar_merkle_distributor::{IndexableLeaf, MerkleDistributor};
use stellar_default_impl_macro::default_impl;

type Distributor = MerkleDistributor<Sha256>;

#[contracttype]
struct Receiver {
    pub index: u32,
    pub address: Address,
    pub amount: i128,
}

impl IndexableLeaf for Receiver {
    fn index(&self) -> u32 {
        self.index
    }
}

#[contracttype]
enum AirdropDataKey {
    MerkleRoot,
}

#[contract]
pub struct AirdropToken;

#[contractimpl]
impl AirdropToken {
    pub fn __constructor(e: &Env, merkle_root: BytesN<32>) {
        Base::set_metadata(e, 18, String::from_str(e, "AirdropToken"), String::from_str(e, "ATK"));
        Distributor::set_root(e, merkle_root);
    }

    pub fn is_claimed(e: &Env, index: u32) -> bool {
        Distributor::is_claimed(e, index)
    }

    pub fn claim(e: &Env, index: u32, receiver: Address, amount: i128, proof: Vec<BytesN<32>>) {
        receiver.require_auth();
        let data = Receiver { index, address: receiver.clone(), amount };
        Distributor::verify_and_set_claimed(e, data, proof);
        Base::mint(e, &receiver, amount);
    }
}

#[default_impl]
#[contractimpl]
impl FungibleToken for AirdropToken {
    type ContractType = Base;
}
