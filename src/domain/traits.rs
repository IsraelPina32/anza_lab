pub trait ContaRepository: Send + Sync {
    fn buscar_saldo(&self, conta: &str) -> Option<u64>;
    fn atualizar_saldo(&self, conta: &str, novo_saldo: u64) -> Result<(), String>;
}