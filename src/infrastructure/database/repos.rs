use crate::domain::traits::ContaRepository;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct InMemoryContaRepository {
    saldos: Arc<RwLock<HashMap<String, u64>>>,
}

impl InMemoryContaRepository {
    pub fn new() -> Self {
        let mut map = HashMap::new();
        map.insert("conta1".to_string(), 5000);
        map.insert("conta2".to_string(), 5000);

        Self {
            saldos: Arc::new(RwLock::new(map)),
        }
    }
}

impl ContaRepository for InMemoryContaRepository {
    fn buscar_saldo(&self, conta: &str) -> Option<u64> {
        let saldos = self.saldos.read().ok()?;
        saldos.get(conta).copied()
    }

    fn atualizar_saldo(&self, conta: &str, novo_saldo: u64) -> Result<(), String> {
        let mut saldos = self.saldos.write().map_err(|e| e.to_string())?;
        saldos.insert(conta.to_string(), novo_saldo);
        Ok(())
    }
}