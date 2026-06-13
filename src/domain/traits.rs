pub trait ContaRepository: Send + Sync {
    
    fn obter_saldo(&self, id: &str) -> Result<u64, String>;

    fn executar_transferencia(&self, de: &str, para: &str, valor: u64) -> Result<(), String>;
}