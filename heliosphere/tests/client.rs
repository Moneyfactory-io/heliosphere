use std::{str::FromStr, sync::LazyLock};

use futures::future::join_all;
use heliosphere::RpcClient;
use heliosphere_core::transaction::TransactionId;

const API: &str = "https://api.shasta.trongrid.io";
const TRANSACTION_ID: &str = "8d9fa8690be0cd307c56cc64606dcd404cc9d2fa1855b7a01ffc9eb57f27e7e7";
static CLIENT: LazyLock<RpcClient> = LazyLock::new(|| RpcClient::new(API).unwrap());

#[tokio::test]
async fn test_eth_get_block() {
    let block_number = CLIENT.eth_block_number().await.unwrap();

    println!("block number: {}", block_number.result);
}

#[tokio::test]
async fn test_get_transaction_info_by_block_num() {
    let block = CLIENT.get_latest_block().await.unwrap();

    println!("block number: {}", block.block_number());

    let txs_info = CLIENT
        .get_tx_info_by_block_num(block.block_number())
        .await
        .unwrap();

    println!("{:#?}", &txs_info);
    println!("Transaction count: {}", txs_info.len());
}

#[tokio::test]
async fn test_get_transaction_info_by_id() {
    let transaction_id = TransactionId::from_str(TRANSACTION_ID).unwrap();

    println!("tx_id: {}", transaction_id);

    let tx_info = CLIENT.get_tx_info_by_id(transaction_id).await.unwrap();

    println!("{:#?}", tx_info);
}

#[tokio::test]
async fn test_get_transactions_info_from_last_block() {
    let block = CLIENT.get_latest_block().await.unwrap();

    println!("Block: {}", block.block_number());
    println!(
        "Transactions to request:\n{:#?}",
        block
            .transactions
            .iter()
            .map(|x| x.tx_id.to_string())
            .collect::<Vec<_>>()
    );

    let txs_info: Vec<_> = block
        .transactions
        .iter()
        .map(|x| CLIENT.get_tx_info_by_id(x.tx_id))
        .collect();

    let res: Vec<_> = join_all(txs_info)
        .await
        .into_iter()
        .map(|x| x.unwrap())
        .collect();

    println!("Tranactions info:\n{:#?}", res);
}
