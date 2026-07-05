use reqwest::{Client};
use axum::{Router, routing::get};
use axum::extract::{State};
use crate::model::raw_mempool::RPCResponse;

const BITCOIN: &str= "http://127.0.0.1:8332"; 

pub async fn route() -> Router {
    let app = Router::new(); 
    let state = Client::new(); 
    app.route("/model/raw_mempool", get(raw_mempool))
    .with_state(state)
}

pub async fn send_request(
    client: &reqwest::Client,
    body: serde_json::Value,
) -> Result <RPCResponse, reqwest::Error> {
    client
        .post(BITCOIN)
        .json(&body)
        .send()
        .await?
        .json::<RPCResponse>()
        .await
}

pub async fn raw_mempool(
    client: &reqwest::Client, 
    body: serde_json::Value,
) -> Result <RPCResponse, reqwest::Error> {
    let body = serde_json::json!({ 
        "jsonrpc": "2.0",
        "method": "getrawmempool",
        "params": [],
        "id": 1,
    });
    send_request(&client, body).await
}