use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transferencia {
    pub de:String,
    pub para: String,
    pub valor: u64,
}

#[derive(Clone)]
pub struct AppState {
    pub contas: Arc<RwLock<HashMap<String, u64>>>,
}

impl AppState {
    pub fn new() -> Self {
        let mut massa_de_dados = HashMap::new();
        massa_de_dados.insert("israel_wallet".to_string(), 100000);
        massa_de_dados.insert("anza_vault".to_string(), 500000);

        Self { contas: Arc::new(RwLock::new(massa_de_dados)) }
    }
}