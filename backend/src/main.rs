use axum::{
    routing::get, 
    Router,
}; 
mod model; 
mod routes; 

#[tokio::main]
pub async fn main() {
    let app = Router::new(); 
    app.merge(routes::block_count_route::route())
    .merge(routes::mempool_info_route::route())
    .merge(routes::estimate_smart_fee_route::route())
    .merge(routes::raw_transaction_route::route())
    .merge(routes::raw_mempool_route::route());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap(); 
    axum::serve(listener, app).await.unwrap(); 
}