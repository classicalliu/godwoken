use anyhow::{anyhow, Result};
use gw_common::H256;
use gw_types::packed::LogItem;
use gw_types::prelude::*;
use std::{convert::TryInto, usize};

pub const GW_LOG_SUDT_TRANSFER: u8 = 0x0;
pub const GW_LOG_SUDT_PAY_FEE: u8 = 0x1;
pub const GW_LOG_POLYJUICE_SYSTEM: u8 = 0x2;
pub const GW_LOG_POLYJUICE_USER: u8 = 0x3;
#[derive(Default, Debug)]
pub struct PolyjuiceArgs {
    pub is_create: bool,
    pub gas_limit: u64,
    pub gas_price: u128,
    pub value: u128,
    pub input: Option<Vec<u8>>,
}

impl PolyjuiceArgs {
    // https://github.com/nervosnetwork/godwoken-polyjuice/blob/v0.6.0-rc1/polyjuice-tests/src/helper.rs#L322
    pub fn decode(args: &[u8]) -> anyhow::Result<Self> {
        let is_create = args[7] == 3u8;
        let gas_limit = u64::from_le_bytes(args[8..16].try_into()?);
        let gas_price = u128::from_le_bytes(args[16..32].try_into()?);
        let value = u128::from_be_bytes(args[32..48].try_into()?);
        let input_size = u32::from_le_bytes(args[48..52].try_into()?);
        let input: Vec<u8> = args[52..(52 + input_size as usize)].to_vec();
        Ok(PolyjuiceArgs {
            is_create,
            // is_static,
            gas_limit,
            gas_price,
            value,
            input: Some(input),
        })
    }
}

pub fn account_id_to_eth_address(account_script_hash: H256, id: u32, ethabi: bool) -> Vec<u8> {
    let offset = if ethabi { 12 } else { 0 };
    let mut data = vec![0u8; offset + 20];
    data[offset..offset + 16].copy_from_slice(&account_script_hash.as_slice()[0..16]);
    data[offset + 16..offset + 20].copy_from_slice(&id.to_le_bytes()[..]);
    data
}

#[derive(Debug, Clone)]
pub enum GwLog {
    SudtTransfer {
        sudt_id: u32,
        from_id: u32,
        to_id: u32,
        amount: u128,
    },
    SudtPayFee {
        sudt_id: u32,
        from_id: u32,
        block_producer_id: u32,
        amount: u128,
    },
    PolyjuiceSystem {
        gas_used: u64,
        cumulative_gas_used: u64,
        created_id: u32,
        status_code: u32,
    },
    PolyjuiceUser {
        address: [u8; 20],
        data: Vec<u8>,
        topics: Vec<H256>,
    },
}

fn parse_sudt_log_data(data: &[u8]) -> (u32, u32, u128) {
    let mut u32_bytes = [0u8; 4];
    u32_bytes.copy_from_slice(&data[0..4]);
    let from_id = u32::from_le_bytes(u32_bytes);

    u32_bytes.copy_from_slice(&data[4..8]);
    let to_id = u32::from_le_bytes(u32_bytes);

    let mut u128_bytes = [0u8; 16];
    u128_bytes.copy_from_slice(&data[8..24]);
    let amount = u128::from_le_bytes(u128_bytes);
    (from_id, to_id, amount)
}

pub fn parse_log(item: &LogItem) -> Result<GwLog> {
    let service_flag: u8 = item.service_flag().into();
    let raw_data = item.data().raw_data();
    let data = raw_data.as_ref();
    match service_flag {
        GW_LOG_SUDT_TRANSFER => {
            let sudt_id: u32 = item.account_id().unpack();
            if data.len() != (4 + 4 + 16) {
                return Err(anyhow!("Invalid data length: {}", data.len()));
            }
            let (from_id, to_id, amount) = parse_sudt_log_data(data);
            Ok(GwLog::SudtTransfer {
                sudt_id,
                from_id,
                to_id,
                amount,
            })
        }
        GW_LOG_SUDT_PAY_FEE => {
            let sudt_id: u32 = item.account_id().unpack();
            if data.len() != (4 + 4 + 16) {
                return Err(anyhow!("Invalid data length: {}", data.len()));
            }
            let (from_id, block_producer_id, amount) = parse_sudt_log_data(data);
            Ok(GwLog::SudtPayFee {
                sudt_id,
                from_id,
                block_producer_id,
                amount,
            })
        }
        GW_LOG_POLYJUICE_SYSTEM => {
            if data.len() != (8 + 8 + 4 + 4 + 4) {
                return Err(anyhow!(
                    "invalid system log raw data length: {}",
                    data.len()
                ));
            }

            let mut u64_bytes = [0u8; 8];
            u64_bytes.copy_from_slice(&data[0..8]);
            let gas_used = u64::from_le_bytes(u64_bytes);
            u64_bytes.copy_from_slice(&data[8..16]);
            let cumulative_gas_used = u64::from_le_bytes(u64_bytes);

            let mut u32_bytes = [0u8; 4];
            u32_bytes.copy_from_slice(&data[16..20]);
            let created_id = u32::from_le_bytes(u32_bytes);
            u32_bytes.copy_from_slice(&data[20..24]);
            let status_code = u32::from_le_bytes(u32_bytes);
            Ok(GwLog::PolyjuiceSystem {
                gas_used,
                cumulative_gas_used,
                created_id,
                status_code,
            })
        }
        GW_LOG_POLYJUICE_USER => {
            let mut offset: usize = 0;
            let mut address = [0u8; 20];
            address.copy_from_slice(&data[offset..offset + 20]);
            offset += 20;
            let mut data_size_bytes = [0u8; 4];
            data_size_bytes.copy_from_slice(&data[offset..offset + 4]);
            offset += 4;
            let data_size: u32 = u32::from_le_bytes(data_size_bytes);
            let mut log_data = vec![0u8; data_size as usize];
            log_data.copy_from_slice(&data[offset..offset + (data_size as usize)]);
            offset += data_size as usize;
            log::debug!("data_size: {}", data_size);

            let mut topics_count_bytes = [0u8; 4];
            topics_count_bytes.copy_from_slice(&data[offset..offset + 4]);
            offset += 4;
            let topics_count: u32 = u32::from_le_bytes(topics_count_bytes);
            let mut topics = Vec::new();
            log::debug!("topics_count: {}", topics_count);
            for _ in 0..topics_count {
                let mut topic = [0u8; 32];
                topic.copy_from_slice(&data[offset..offset + 32]);
                offset += 32;
                topics.push(topic.into());
            }
            if offset != data.len() {
                return Err(anyhow!(
                    "Too many bytes for polyjuice user log data: offset={}, data.len()={}",
                    offset,
                    data.len()
                ));
            }
            Ok(GwLog::PolyjuiceUser {
                address,
                data: log_data,
                topics,
            })
        }
        _ => Err(anyhow!("invalid log service flag: {}", service_flag)),
    }
}
