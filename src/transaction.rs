#[derive(Clone)]
pub struct Transaction {
    pub from: String,

    // ⭐ 合约地址
    pub to: String,

    // ⭐ 方法名
    pub method: String,

    // ⭐ 参数
    pub args: Vec<String>,
}