use serde::{Deserialize, Serialize};

/// RPC Response wrapper
#[derive(Debug, Clone, Deserialize, Hash)]
pub struct RpcResponse<T> {
    /// JSON-RPC Version
    #[serde(rename = "jsonrpc")]
    pub json_rpc: String,
    /// Id
    pub id: u64,
    /// Result of RPC Call
    pub result: T,
}

/// RPC Request wrapper
#[derive(Debug, Clone, Serialize, Hash)]
pub struct RpcPayload<T: Serialize> {
    /// JSON-RPC Version
    #[serde(rename = "jsonrpc")]
    pub json_rpc: String,
    /// RPC Method
    pub method: String,
    /// Id
    pub id: u64,
    /// Params for call
    pub params: T,
}

impl<T: Serialize> RpcPayload<T> {
    /// Initial Request wrapper
    pub fn init(method: String, params: T) -> RpcPayload<T> {
        RpcPayload {
            json_rpc: "2.0".to_string(),
            id: 64,
            method,
            params,
        }
    }
}

/// Information about block
#[derive(Debug, Clone, Serialize, Hash)]
pub struct Block {}
