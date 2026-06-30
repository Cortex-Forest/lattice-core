use std::collections::HashMap;
use crate::contract::Contract;

pub struct VM {
    pub contracts: HashMap<String, Contract>,
}

impl VM {
    pub fn new() -> Self {
        Self {
            contracts: HashMap::new(),
        }
    }

    // ⭐ deploy 合约
    pub fn deploy(&mut self, address: String) {
        let c = Contract::new(address.clone());
        self.contracts.insert(address, c);
    }

    // ⭐ call 合约
    pub fn call(&mut self, address: &str, method: &str, args: Vec<String>) -> String {
        if let Some(contract) = self.contracts.get_mut(address) {
            contract.call(method, args)
        } else {
            "contract not found".to_string()
        }
    }
}