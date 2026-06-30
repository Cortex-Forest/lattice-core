use crate::vm::VM;
use crate::transaction::Transaction;

pub struct Blockchain {
    pub vm: VM,
}

impl Blockchain {
    pub fn new() -> Self {
        Self {
            vm: VM::new(),
        }
    }

    pub fn process_tx(&mut self, tx: Transaction) -> String {
        self.vm.call(&tx.to, &tx.method, tx.args)
    }
}