use rust_decimal::Decimal;
use sqlx::types::chrono::{DateTime, NaiveDateTime, Utc};
#[derive(sqlx::FromRow)]
pub struct Block {
    pub number: Decimal,
    pub hash: String,
    pub parent_hash: String,
    pub logs_bloom: String,
    pub gas_limit: Decimal,
    pub gas_used: Decimal,
    pub miner: String,
    pub size: Decimal,
    pub timestamp: NativeDateTime,
}

#[derive(sqlx::FromRow)]
pub struct Transaction {
    pub hash: String,
    pub block_number: Decimal,
    pub block_hash: String,
    pub transaction_index: i32,
    pub from_address: String,
    pub to_address: Option<String>,
    pub value: Decimal,
    pub nonce: Decimal,
    pub gas_limit: Decimal,
    pub gas_price: Decimal,
    pub input: String,
    pub v: String,
    pub r: String,
    pub s: String,
    pub cumulative_gas_used: Decimal,
    pub gas_used: Decimal,
    pub logs_bloom: String,
    pub contract_address: Option<String>,
    pub status: bool,
}

#[derive(sqlx::FromRow)]
pub struct Log {
    pub transaction_id: i64,
    pub transaction_hash: String,
    pub transaction_index: i32,
    pub block_number: i64,
    pub block_hash: String,
    pub address: String,
    pub data: String,
    pub log_index: i32,
    pub topics: Vec<String>,
}
