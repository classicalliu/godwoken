use crate::utils;
use anyhow::Result;
use hex;
use rand::Rng;
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
    thread, time,
};

const MIN_WALLET_CAPACITY: f32 = 100000.0f32;

#[derive(Debug)]
struct NodeWalletInfo {
    node_name: String,
    privkey_path: PathBuf,
    testnet_address: String,
    lock_hash: String,
    lock_arg: String,
    block_assembler_code_hash: String,
}

pub fn prepare_pk(
    payer_privkey: &Path,
    ckb_count: u32,
    nodes_count: u8,
    output_dir: &Path,
    _poa_config_path: &Path,
    _rollup_config_path: &Path,
) -> Result<()> {
    let nodes_privkeys = prepare_privkeys(output_dir, nodes_count);
    let _nodes_info = check_wallets_info(nodes_privkeys, ckb_count, payer_privkey);
    generate_poa_config();
    generate_rollup_config();
    Ok(())
}

fn prepare_privkeys(output_dir: &Path, nodes_count: u8) -> HashMap<String, PathBuf> {
    (0..nodes_count)
        .map(|index| {
            let node_name = format!("node{}", index.to_string());
            let node_dir = utils::make_path(output_dir, vec![&node_name]);
            fs::create_dir_all(&node_dir).expect("create node dir");
            let privkey_file = utils::make_path(&node_dir, vec!["pk"]);
            let privkey = fs::read(&privkey_file).unwrap_or(Vec::new());
            if !privkey.starts_with(b"0x")
                || privkey.len() != 66
                || hex::decode(&privkey[2..]).is_err()
            {
                generate_privkey_file(&privkey_file);
            }
            (node_name, privkey_file)
        })
        .collect()
}

fn check_wallets_info(
    nodes_privkeys: HashMap<String, PathBuf>,
    ckb_count: u32,
    payer_privkey_path: &Path,
) -> Vec<NodeWalletInfo> {
    nodes_privkeys
        .into_iter()
        .map(|(node, privkey)| {
            let wallet_info = get_wallet_info(&node, privkey);
            let mut capacity = query_wallet_capacity(&wallet_info.testnet_address);
            log::info!("{}'s wallet capacity: {}", node, capacity);
            if capacity < MIN_WALLET_CAPACITY {
                log::info!("Transfer ckb...");
                transfer_ckb(&wallet_info, payer_privkey_path, ckb_count);
                thread::sleep(time::Duration::from_secs(30));
                capacity = query_wallet_capacity(&wallet_info.testnet_address);
                assert!(
                    capacity >= MIN_WALLET_CAPACITY,
                    "wallet haven't received ckb, please try again"
                );
                log::info!("{}'s wallet capacity: {}", node, capacity);
            }
            wallet_info
        })
        .collect()
}

fn generate_poa_config() {}

fn generate_rollup_config() {}

fn generate_privkey_file(privkey_file_path: &Path) {
    let key = rand::thread_rng().gen::<[u8; 32]>();
    let privkey = format!("0x{}", hex::encode(key));
    fs::write(&privkey_file_path, &privkey).expect("create pk file");
}

fn get_wallet_info(node_name: &str, privkey_path: PathBuf) -> NodeWalletInfo {
    let (stdout, stderr) = utils::run_in_output_mode(
        "ckb-cli",
        vec![
            "util",
            "key-info",
            "--privkey-path",
            &privkey_path.display().to_string(),
        ],
    )
    .expect("get key info");
    NodeWalletInfo {
        node_name: node_name.into(),
        privkey_path,
        testnet_address: look_after_in_line(&stdout, "testnet:"),
        lock_hash: look_after_in_line(&stdout, "lock_hash:"),
        lock_arg: look_after_in_line(&stdout, "lock_arg:"),
        block_assembler_code_hash: look_after_in_line(&stderr, "code_hash ="),
    }
}

fn query_wallet_capacity(address: &str) -> f32 {
    let (stdout, _) = utils::run_in_output_mode(
        "ckb-cli",
        vec!["wallet", "get-capacity", "--address", address],
    )
    .expect("query wallet capacity");
    look_after_in_line(&stdout, "total:")
        .split(' ')
        .collect::<Vec<&str>>()[0]
        .parse::<f32>()
        .expect("parse capacity")
}

fn transfer_ckb(node_wallet: &NodeWalletInfo, payer_privkey_path: &Path, ckb_count: u32) {
    utils::run(
        "ckb-cli",
        vec![
            "wallet",
            "transfer",
            "--to-address",
            &node_wallet.testnet_address,
            "--capacity",
            &ckb_count.to_string(),
            "--tx-fee",
            "1",
            "--privkey-path",
            &payer_privkey_path.display().to_string(),
        ],
    )
    .expect("transfer ckb");
}

fn look_after_in_line(text: &str, key: &str) -> String {
    text.split(key).collect::<Vec<&str>>()[1]
        .split('\n')
        .collect::<Vec<&str>>()[0]
        .trim_matches(&['"', ' '][..])
        .to_owned()
}
