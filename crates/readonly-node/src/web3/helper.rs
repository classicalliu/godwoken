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
        let is_create = true;
        let is_static = true;
        let gas_limit = 0u64;
        let gas_price = 0u128;
        let value = 0u128;
        let input: Vec<u8> = vec![1, 2, 3, 4];
        Ok(Self {
            is_create: is_create,
            is_static: is_static,
            gas_limit: gas_limit,
            gas_price: gas_price,
            value: value,
            input: Some(input),
        })
    }
}
