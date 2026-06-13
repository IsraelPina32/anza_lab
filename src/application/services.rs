use crate::domain::traits::ContaRepository;

pub struct TransferenciaService<R: ContaRepository> {
    repo: R
}

impl<R: ContaRepository> TransferenciaService<R> {
     pub fn new(repo: R) -> Self {
        Self {repo}
     }

     pub fn processar_transferencia(&self, de: &str, para: &str, valor:u64) -> Result<(), String>{
        if de == para {
            return Err("A conta de origem não pode ser igual à de destino.".to_string());
        }

        let saldo_origem = self.repo.obter_saldo(de)?;

        if saldo_origem < valor {
            return Err(format!(
                "Saldo insuficiente para executar a operação. Saldo atual: {}", 
                saldo_origem
            ));
        }

        self.repo.executar_transferencia(de, para, valor)
   
     }
}