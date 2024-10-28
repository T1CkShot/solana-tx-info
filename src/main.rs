use dotenv::dotenv;
use solana_client::rpc_client::{GetConfirmedSignaturesForAddress2Config, RpcClient};
use solana_client::rpc_config::RpcTransactionConfig;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;
use std::env;
use std::error::Error;
use std::str::FromStr;

async fn get_latest_transaction_for_wallet(wallet_address: &str) -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let rpc_url = env::var("RPC_URL")
        .map_err(|e| format!("Failed to retrieve RPC_URL from environment: {}", e))?;
    let client = RpcClient::new(rpc_url);

    let wallet_pubkey = Pubkey::from_str(wallet_address)
        .map_err(|e| format!("Invalid wallet address '{}': {}", wallet_address, e))?;

    let signatures = client
        .get_signatures_for_address_with_config(
            &wallet_pubkey,
            GetConfirmedSignaturesForAddress2Config {
                limit: Some(1),
                ..Default::default()
            },
        )
        .map_err(|e| {
            format!(
                "Failed to fetch signatures for wallet '{}': {}",
                wallet_address, e
            )
        })?;

    if let Some(signature_info) = signatures.last() {
        let signature = signature_info.signature.parse::<Signature>().map_err(|e| {
            format!(
                "Failed to parse signature '{}': {}",
                signature_info.signature, e
            )
        })?;

        let transaction_config = RpcTransactionConfig {
            max_supported_transaction_version: Some(0),
            ..Default::default()
        };

        let transaction = client
            .get_transaction_with_config(&signature, transaction_config)
            .map_err(|e| {
                format!(
                    "Failed to get transaction details for signature '{}': {}",
                    signature, e
                )
            })?;

        println!(
            "Latest Transaction Signature: {:?}",
            signature_info.signature
        );
        println!("Transaction Details: {:#?}", transaction);
    } else {
        println!("No transactions found for the wallet.");
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <WALLET_ADDRESS>", args[0]);
        std::process::exit(1);
    }

    let wallet_address = &args[1];

    match get_latest_transaction_for_wallet(wallet_address).await {
        Ok(_) => println!("Latest transaction retrieval successful."),
        Err(e) => eprintln!("Error: {}", e),
    }
}
