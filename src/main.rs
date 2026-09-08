mod application;
mod domain;
mod infrastructure;

use crate::application::services::TransferenciaService;
use crate::domain::models::Transferencia;
use crate::infrastructure::database::repos::InMemoryContaRepository;
use axum::{Json, Router, extract::State, routing::post};
use std::sync::Arc;


struct AppState {
    service: TransferenciaService<InMemoryContaRepository>,
}

#[tokio::main]
async fn main() {
    let repo = InMemoryContaRepository::new();
    let service = TransferenciaService::new(repo);
    let shared_state = Arc::new(AppState { service });

    let app = Router::new()
        .route("/transferir", post(handle_transferencia))
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Servidor rodando em http://localhost:3000");
    axum::serve(listener, app).await.unwrap();

}
async fn handle_transferencia(
        State(state): State<Arc<AppState>>,
        Json(payload): Json<Transferencia>,
    ) -> String {
        match state
            .service
            .processar_transferencia(&payload.de, &payload.para, payload.valor)
        {
            Ok(_) => "Transferência realizada com sucesso!".to_string(),
            Err(e) => format!("Erro na transferência: {}", e),
        }
    }