use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Transaction {
    #[serde(rename = "type")]
    pub tx_type: String,
    pub client: u16,
    pub tx: u32,
    pub amount: Option<f64>,
}