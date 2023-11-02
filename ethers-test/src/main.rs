use ethers::prelude::*;
use ethers::providers::{Provider, Http};
use std::sync::Arc;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let contract_address = "0xfba3912ca04dd458c843e2ee08967fc04f3579c2".parse::<Address>()?;
    abigen!(IERC721, "./src/abi.json");
    let rpc_url = format!("https://goerli.infura.io/v3/{}", env::var("INFURA_API_KEY")?);
    let provider = Provider::<Http>::try_from(rpc_url.as_str())?;
    let provider = Arc::new(provider);
    let contract = IERC721::new(contract_address, provider.clone());

    let function_name = "symbol";
    let function_params = ();
    let result: String = contract.method(function_name, function_params)?.call().await?;
    println!("{}", result);
    Ok(())
}