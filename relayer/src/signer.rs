use alloy::{
    network::EthereumWallet,
    primitives::{Address, Bytes, B256},
    providers::ProviderBuilder,
    signers::local::PrivateKeySigner,
    sol,
};
use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use std::env;

sol! {
    #[sol(rpc)]
    contract ZkMixer {
        function withdraw(bytes proof, bytes32 root, bytes32 nullifierHash, address recipient);
    }
}

#[derive(Deserialize)]
pub struct WithdrawRequest {
    pub proof: Bytes,
    pub root: B256,
    #[serde(alias = "nullifier_hash")]
    pub nullifier_hash: B256,
    pub recipient: Address,
}

#[derive(Serialize)]
pub struct WithdrawResponse {
    pub tx_hash: String,
}

// signs, broadcasts, and returns the tx hash (or a 500 with the error).
pub async fn relay_withdraw(
    Json(req): Json<WithdrawRequest>,
) -> Result<Json<WithdrawResponse>, (StatusCode, String)> {
    match relay(req).await {
        Ok(tx_hash) => Ok(Json(WithdrawResponse { tx_hash })),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

async fn relay(req: WithdrawRequest) -> anyhow::Result<String> {
    let signer: PrivateKeySigner = env::var("RELAYER_PRIVATE_KEY")?.parse()?;
    let wallet = EthereumWallet::from(signer);

    let rpc_url = env::var("RELAYER_RPC_URL")?.parse()?;
    let provider = ProviderBuilder::new().wallet(wallet).connect_http(rpc_url);

    let contract_address: Address = env::var("RELAYER_CONTRACT_ADDRESS")?.parse()?;
    let zk_mixer = ZkMixer::new(contract_address, provider);

    let pending = zk_mixer
        .withdraw(req.proof, req.root, req.nullifier_hash, req.recipient)
        .send()
        .await?;

    let receipt = pending.get_receipt().await?;
    Ok(receipt.transaction_hash.to_string())
}
