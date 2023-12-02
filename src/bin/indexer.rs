use dotenvy::dotenv;
use ethers::types::{H160, U256};
use eyre::Result;
use std::thread;
use std::{env, time::Duration};

use ethers::{
    providers::{Http, Middleware, Provider},
    types::{Address, Filter},
};
#[tokio::main]
async fn main() -> Result<()> {
    dotenv().expect("Could not load .env file");

    const GNARS_ADDRESS: &str = "0x558BFFF0D583416f7C4e380625c7865821b8E95C";

    let provider_url = env::var("PROVIDER_URL").expect("PROVIDER_URL required");

    let provider = Provider::<Http>::try_from(provider_url).expect("could not load provider");

    let curr_block = &provider.get_block_number().await?;
    let client = &provider;

    let mut start_block: u64 = 14998510;
    let mut end_block: u64 = start_block + 2000;

    loop {
        let filter = Filter::new()
            .address(GNARS_ADDRESS.parse::<Address>()?)
            .event("Transfer(address,address,uint256)")
            .from_block(start_block)
            .to_block(end_block);

        let logs = client.get_logs(&filter).await?;

        for log in &logs {
            println!("tx hash {:?}", log.block_hash.unwrap());
            println!("from: {:#?}", H160::from(log.topics[1]));
            println!("to: {:#?}", H160::from(log.topics[2]));
            println!(
                "token id: {:#?}",
                U256::from_big_endian(log.topics[3].as_bytes())
            );
            println!("");
        }

        println!(
            "total logs found: {}, start block: {}, end block: {}",
            logs.iter().len(),
            start_block,
            end_block
        );

        start_block = end_block + 2000;
        end_block = start_block + 2000;

        if end_block >= curr_block.as_u64() {
            println!("reached the end");
            break;
        }

        thread::sleep(Duration::from_millis(50));
    }

    Ok(())
}
