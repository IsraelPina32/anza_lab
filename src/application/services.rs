use crate::domain::traits::ContaRepository;

pub struct TransferenciaService<R: ContaRepository> {
    repo: R,
}

impl<R: ContaRepository> TransferenciaService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub fn processar_transferencia(&self, de: &str, para: &str, valor: u64) -> Result<(), String> {
        if de == para {
            return Err("A conta de origem não pode ser igual à de destino.".to_string());
        }

        let saldo_de = self
            .repo
            .buscar_saldo(de)
            .ok_or_else(|| "Conta de origem não encontrada".to_string())?;

        let saldo_para = self
            .repo
            .buscar_saldo(para)
            .ok_or_else(|| "Conta de destino não encontrada".to_string())?;

        if saldo_de < valor {
            return Err(format!(
                "Saldo insuficiente para executar a operação. Saldo atual: {}",
                saldo_de
            ));
        }

        self.repo.atualizar_saldo(de, saldo_de - valor)?;
        self.repo.atualizar_saldo(para, saldo_para + valor)?;

        Ok(())
    }

    pub fn obter_saldo(&self, conta: &str) -> Option<u64> {
        self.repo.buscar_saldo(conta)
    }
}