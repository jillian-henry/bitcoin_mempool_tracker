use axum::{
    routing::get, 
    Router,
}; 
mod model; 
mod routes; 

#[tokio::main]
pub async fn main() {
    let app = Router::new(); 
    app.merge(block_count_route)
    .merge(mempool_info_route)
    .merge(estimate_smart_fee_route)
    .merge(raw_transaction_route)
    .merge(raw_mempool_route)

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap(); 
    axum::serve(listener, app).await.unwrap(); 
}