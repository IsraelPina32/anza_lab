use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transferencia {
    pub de:String,
    pub para: String,
    pub valor: u64,
}