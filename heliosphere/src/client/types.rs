use std::collections::BTreeMap;

use heliosphere_core::{
    transaction::{Transaction, TransactionId},
    Address,
};
use serde::{Deserialize, Serialize};

/// Result
#[derive(Deserialize, Debug, Clone)]
pub struct ResponseResult {
    /// Is successful
    pub result: bool,
    /// response code, an enum type
    #[serde(default)]
    pub code: Option<String>,
    /// Result message
    #[serde(default)]
    pub message: Option<String>,
}

/// Broadcast transaction response
#[derive(Deserialize, Debug, Clone)]
pub struct BroadcastTxResponse {
    /// Error code
    #[serde(default)]
    pub code: Option<String>,
    /// Detailed error information
    #[serde(default)]
    pub message: Option<String>,
    /// Transaction id
    pub txid: TransactionId,
}

/// Trigger constant contract response
#[derive(Deserialize, Debug, Clone)]
pub struct QueryContractResponse {
    /// Run result
    pub result: ResponseResult,
    /// Result list
    #[serde(default)]
    pub constant_result: Vec<String>,
    /// Estimated energy consumption, including the basic energy consumption and penalty energy consumption
    #[serde(default)]
    pub energy_used: u64,
}

impl QueryContractResponse {
    /// Result
    pub fn constant_result(&self, index: usize) -> Result<Vec<u8>, crate::Error> {
        let res = self
            .constant_result
            .get(index)
            .ok_or(crate::Error::InvalidIndex)?;
        hex::decode(res).map_err(|e| crate::Error::UnknownResponse(e.to_string()))
    }
}

/// Trigger contract response
#[derive(Deserialize, Debug, Clone)]
pub struct TriggerContractResponse {
    /// Transaction information, refer to GetTransactionByID
    pub transaction: Transaction,
}

/// Resource type: Energy or Bandwidth
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "UPPERCASE")]
pub enum ResourceType {
    /// Bandwidth resource
    Bandwidth,
    /// Energy resource
    Energy,
}

/// Account resources
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AccountResources {
    /// Free bandwidth used
    #[serde(rename = "freeNetUsed", default)]
    pub free_net_used: u64,
    /// Total free bandwidth
    #[serde(rename = "freeNetLimit", default)]
    pub free_net_limit: u64,
    /// Used amount of bandwidth obtained by staking
    #[serde(default)]
    pub net_used: u64,
    /// Total bandwidth obtained by staking
    #[serde(default)]
    pub net_limit: u64,
    /// Total bandwidth that can be obtained by staking
    #[serde(default)]
    pub total_net_limit: u64,
    /// Total TRX staked for bandwidth
    #[serde(default)]
    pub total_net_weight: u64,
    /// TRON Power(vote)
    #[serde(rename = "tronPowerLimit", default)]
    pub tron_power_limit: u64,
    /// Energy used
    #[serde(default)]
    pub energy_used: u64,
    /// Total energy obtained by staking
    #[serde(default)]
    pub energy_limit: u64,
    /// Total energy that can be obtained by staking
    #[serde(default)]
    pub total_energy_limit: u64,
    /// Total TRX staked for energy
    #[serde(default)]
    pub total_energy_weight: u64,
}

/// Transaction receipt
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionRet {
    /// Transaction Execution Result
    #[serde(rename = "contractRet")]
    pub contract_ret: String,
}

/// Transaction from solidity wallet
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolidityTransactionInfo {
    #[serde(flatten)]
    /// Transaction
    pub transaction: Transaction,
    /// Transaction Execution Results
    pub ret: Vec<TransactionRet>,
}

/// Chain parameter (key, value)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ChainParameter {
    /// parameter name
    pub key: String,
    /// parameter value
    #[serde(default)]
    pub value: Option<i64>,
}

/// Chain parameters as returned by GetChainParameters
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ChainParametersResponse {
    /// A list of dynamic parameter objects
    #[serde(rename = "chainParameter")]
    pub chain_parameter: Vec<ChainParameter>,
}

/// Account info (as returned by /wallet/getaccount)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AccountBalanceResponse {
    /// TRX balance
    #[serde(default)]
    pub balance: Option<u64>,
}

/// Transaction execution result
pub type TransactionResult = String;

/// Transaction receipt, including transaction execution result and transaction fee details
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ResourceReceipt {
    /// The amount of energy consumed in the caller's account
    #[serde(default)]
    pub energy_usage: Option<u64>,
    /// The amount of TRX burned to pay for energy
    #[serde(default)]
    pub energy_fee: Option<u64>,
    /// The amount of energy consumed in the contract deployer's account
    #[serde(default)]
    pub origin_energy_usage: Option<u64>,
    /// The total amount of energy consumed by the transaction
    #[serde(default)]
    pub energy_usage_total: Option<u64>,
    /// The amount of bandwidth consumed
    #[serde(default)]
    pub net_usage: Option<u64>,
    /// The amount of TRX burned to pay for the bandwidth
    #[serde(default)]
    pub net_fee: Option<u64>,
    /// Transaction execution result
    #[serde(default)]
    pub result: Option<TransactionResult>, // TODO: Fix type
    /// The amount of extra energy that needs to be paid for calling a few popular contracts
    #[serde(default)]
    pub energy_penalty_total: Option<u64>,
}

/// The log of events triggered during the smart contract call
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Log {
    /** Contract address. In order to be compatible with EVM, the address in TVM is a hex
     format address without the prefix 0x41, so if you want to parse the address in the log,
     you need to add 41 to the beginning of the log address,
     and then convert it to Base58 format.
    */
    address: Address,
    /// The topic of the event, including the event itself and parameters marked as indexed.
    topics: Vec<String>,
    /// Non-indexed parameters of events.
    data: String,
}

/// Internal transaction
// TODO: Make internal transaction struct
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct InternalTransaction {}

/// Transaction info
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TransactionInfo {
    /// Transaction ID
    pub id: TransactionId,
    /** The total number of TRX burned in this transaction,
     including TRX burned for bandwidth/energy, memo fee,
     account activation fee, multi-signature fee and other fees
    */
    pub fee: u64,
    /// The block number
    #[serde(rename = "blockNumber")]
    pub block_number: u64,
    /// The block timestamp, the unit is millisecond
    #[serde(rename = "blockTimeStamp")]
    pub block_timestamp: u64,
    /// Transaction Execution Results
    #[serde(rename = "contractResult")]
    pub contract_result: Vec<String>,
    /// Contract address
    #[serde(default)]
    pub contract_address: Option<Address>,
    /// Transaction receipt, including transaction execution result and transaction fee details
    pub receipt: ResourceReceipt,
    /// The log of events triggered during the smart contract call
    #[serde(default)]
    pub logs: Option<Vec<Log>>,
    /// Execution results. If the execution is successful, the field will not be displayed
    /// in the returned value, if the execution fails, the field will be "FAILED"
    #[serde(default)]
    pub result: Option<String>,
    /** When the transaction execution fails,
        the details of the failure will be returned through this field.
        Hex format, you can convert it to a string to get plaintext information.
    */
    #[serde(default, rename = "resMessage")]
    pub res_message: Option<String>,
    /** For the withdrawal reward transaction、unfreeze transaction,
        they will withdraw the vote reward to account.
        The number of rewards withdrawn to the account is returned through this field
        and the unit is sun
    */
    #[serde(default)]
    pub withdraw_amount: Option<u64>,
    /** In the Stake1.0 stage, for unstaking transactions,
        this field returns the amount of unstaked TRX,
        the unit is sun
    */
    #[serde(default)]
    pub unfreeze_amount: Option<u64>,
    /// Internal transaction
    #[serde(default)]
    pub internal_transactions: Option<Vec<InternalTransaction>>,
    /** In the Stake2.0 stage, for unstaking transaction and withdrawing unfrozen balance transaction,
        and cancelling all unstakes transaction,
        this field returns the amount of unfrozen TRX withdrawn to the account in this transaction,
        the unit is sun
    */
    #[serde(default)]
    pub withdraw_expire_amount: Option<u64>,
    /** The amount of TRX re-staked to obtain various types of resources,
        in sun, that is, the amount of unstaked principal that has been canceled,
        the key is: "BANDWIDTH" or "ENERGY" or "TRON_POWER"
    */
    // TODO: Add mapping to enum
    #[serde(default, rename = "cancel_unfreezeV2_amount")]
    pub cancel_unfreeze_v2_amount: Option<BTreeMap<String, u64>>,
}
