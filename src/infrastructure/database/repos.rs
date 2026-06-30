use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use crate::domain::traits::ContaRepository;

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
    fn obter_saldo(&self, conta_id: &str) -> Result<u64, String> {
        let saldos = self
            .saldos
            .read()
            .map_err(|_| "Rwlock enveneado  (poisoned) na leitura".to_string())?;

        saldos
            .get(conta_id)
            .copied()
            .ok_or_else(|| format!("Conta {} não encontrada", conta_id))
    }

    fn executar_transferencia(&self, de: &str, para: &str, valor: u64) -> Result<(), String> {
        let mut saldos = self
            .saldos
            .write()
            .map_err(|_| "Rwlock envenenado (poisoned) na escrita".to_string())?;

        let saldo_de = *saldos
            .get(de)
            .ok_or_else(|| format!("Conta {}  não encontrada", de))?;
        let saldo_para = *saldos
            .get(para)
            .ok_or_else(|| format!("Conta {}  não encontrada", para))?;

        saldos.insert(de.to_string(), saldo_de - valor);
        saldos.insert(para.to_string(), saldo_para + valor);

        Ok(())
    }
}
