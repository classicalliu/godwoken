use std::{convert::TryInto, usize};
#[derive(Default, Debug)]
pub struct PolyjuiceArgs {
    pub is_create: bool,
    pub is_static: bool,
    pub gas_limit: u64,
    pub gas_price: u128,
    pub value: u128,
    pub input: Option<Vec<u8>>,
}

impl PolyjuiceArgs {
    pub fn decode(args: &[u8]) -> anyhow::Result<Self> {
        let is_create = if args[0] == 3u8 { true } else { false };
        let is_static = true;
        let gas_limit = u64::from_le_bytes(args[2..10].try_into()?);
        let gas_price = u128::from_le_bytes(args[10..26].try_into()?);
        let value = u128::from_be_bytes(args[26..58].try_into()?);
        let input_size = u32::from_le_bytes(args[58..62].try_into()?);
        let input: Vec<u8> = args[62..(62+input_size as usize)].to_vec();
        Ok(PolyjuiceArgs {
            is_create: is_create,
            is_static: is_static,
            gas_limit: gas_limit,
            gas_price: gas_price,
            value: value,
            input: Some(input),
        })
    }
}
