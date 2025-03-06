use std::sync::LazyLock;

use futures::future::join_all;
use heliosphere::RpcClient;

const API: &str = "https://api.shasta.trongrid.io";
static CLIENT: LazyLock<RpcClient> = LazyLock::new(|| RpcClient::new(API).unwrap());

#[tokio::test]
async fn test_get_transaction_info_by_id() {
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
