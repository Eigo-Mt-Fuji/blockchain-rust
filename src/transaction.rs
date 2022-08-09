use super::*;
use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct Output {
    pub to_addr: Address,
    pub value: u64,
}

pub struct Transaction {

    pub outputs: Vec<Output>,
    pub inputs: Vec<Output>,
}

impl Hashable for Output { 

    fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        // TODO: why not &self? / how does it work vec extend / how does it work as_bytes / what different between as_bytes and bytes.
        bytes.extend(self.to_addr.as_bytes());
        bytes.extend(&u64_bytes(&self.value));
        bytes
    }
}

impl Hashable for Transaction {

    fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        // Vec<Output> to Vec<v8>
        bytes.extend(self.inputs.iter().flat_map( |input| input.bytes()).collect::<Vec<u8>>());
        bytes
    }
}

impl Transaction {

    pub fn is_coinbase(&self) -> bool {
        self.inputs.is_empty()
    }
    pub fn input_value(&self) -> u64 {
        // TODO: why NOT &self.input?
        self.inputs.iter().map( |val| val.value).sum()
    }

    pub fn output_value(&self) -> u64 {
        // TODO: why NOT &self.input?
        self.outputs.iter().map( |val| val.value).sum()
    }

    pub fn input_hashes(&self) -> HashSet<Hash> {
        // TODO: why NOT &self.input?
        self.inputs.iter().map(|val| val.hash() ).collect::<HashSet<Hash>>()
    }
    pub fn output_hashes(&self) -> HashSet<Hash> {
        // TODO: why NOT &self.input?
        self.outputs.iter().map(|val| val.hash() ).collect::<HashSet<Hash>>()
    }
}