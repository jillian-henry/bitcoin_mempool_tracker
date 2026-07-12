use reqwest::{Client};
use axum::{Router, routing::get, Json};
use axum::extract::{State};
use axum::http::StatusCode;
use crate::model::estimate_smart_fee::RPCResponse; 

const BITCOIN: &str = "http://127.0.0.1:8332";

// set up for out routes and data
pub fn route() -> Router {
    let app = Router::new(); // directs traffic 
    let state = Client::new(); 

    app.route("/model/estimate_smart_fee_route/:target", get(estimate_smart_fee_route))
    .with_state(state) // method on the router that attaches shared data that handlers can use
}

pub async fn send_request(
    client: &reqwest::Client,
    body: serde_json::Value
) -> Result <estimate_smart_fee::RPCResponse, reqwest::Error> {
    client
    .post(BITCOIN)
    .json(&body) // fetches the json body
    .send() // sends the request
    .await?
    .json::<estimate_smart_fee::RPCResponse>()
    .await // unwraps the Result 
}

pub async fn estimate_smart_fee_route(State(client):State<Client>, Path(target): Path<u64>
) -> Result<Json<estimate_smart_fee::RPCResponse>, (StatusCode, String)> {

    let body = serde_json::json!(
        {
            "jsonrpc": "2.0",
            "method": "estimatesmartfee",
            "params": [target],
            "id": 1
        }
    );
    send_request(&client, body).await
    .map_err(|e|(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}