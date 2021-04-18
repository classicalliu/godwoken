# Readonly Node

The readonly node would sync L2 block from CKB L1 network, rebuild the chain state, and besides mapping sudt/polyjuice transactions into web3 compatible database.

## Setup

### CKB Network with Godwoken deployment

The CKB network can be a local dev chain or running testnet, with godwoken instance deployed on it. Further reading: https://github.com/nervosnetwork/godwoken.

### CKB Indexer

The [CKB Indexer](https://github.com/nervosnetwork/ckb-indexer) is required for transaction query.

### SQL DB Config

The postgreSQL is suppported so far. You can setup database with [sqlx-cli](https://lib.rs/crates/sqlx-cli)

```
$ cd godwoken/crates/readonly-node
$ cargo install sqlx-cli --no-default-features --features postgres
$ export DATABASE_URL=postgres://username:password@localhost:5432/dbname
$ sqlx database create
$ sqlx migrate run

```

## Start Syncing
```
$ cd godwoken && cargo build
$ ./target/debug/gw-readonly-node --ckb-rpc $CKB_RPC --indexer-rpc $INDEXER_RPC --listen $LISTEN_PORT --runner-config $RUNNER_CONFIG --sql $DATABASE_URL
```