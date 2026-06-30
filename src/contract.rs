use std::collections::HashMap;

#[derive(Clone)]
pub struct Contract {
    pub address: String,
    pub storage: HashMap<String, String>,
}

impl Contract {
    pub fn new(address: String) -> Self {
        Self {
            address,
            storage: HashMap::new(),
        }
    }

    // ⭐ 写入状态
    pub fn set(&mut self, key: String, value: String) {
        self.storage.insert(key, value);
    }

    // ⭐ 读取状态
    pub fn get(&self, key: &str) -> Option<String> {
        self.storage.get(key).cloned()
    }

    // ⭐ 合约调用入口（VM核心）
    pub fn call(&mut self, method: &str, args: Vec<String>) -> String {
        match method {
            "set" => {
                self.set(args[0].clone(), args[1].clone());
                "OK".to_string()
            }
            "get" => {
                self.get(&args[0]).unwrap_or("null".to_string())
            }
            _ => "unknown method".to_string(),
        }
    }
}