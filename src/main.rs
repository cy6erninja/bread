extern crate byteorder;
extern crate hex;
extern crate reqwest;
extern crate serde_json;
extern crate tokio;
use serde_json::{json, Value};
use std::env::{args, Args};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read the amount of addresses that either sent some transaction or received some money on a blockchain.
    let mut a: Args = args();
    let mut startBlockNumber = 11381909;
    let bn = 11381910;
    while startBlockNumber < bn {
        let blockHash = getBlockHash(bn).await.unwrap();
        println!("{:?}", blockHash);

        let bh = getBlockData(blockHash.as_str()).await;
        let decoded = hex::decode(bh.unwrap());

        println!("{:?}", decoded);
        startBlockNumber += 1;
    }

    Ok(())
}

async fn getHeadBlockNumber() -> Result<u32, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let resp = client
        .post("https://rpc.polkadot.io")
        .json(&json! {

            {
                "id": 1,
                "jsonrpc": "2.0",
                "method": "chain_getBlock"
            }

        })
        .send()
        .await?;

    let text: Value = resp.json().await.unwrap();
    let number = &text["result"]["block"]["header"]["number"]
        .as_str()
        .unwrap()
        .trim_start_matches("0x");

    let mut last_block = hex::decode(number).unwrap();

    // Add one more byte to the Vec in order for the size to fit u34 (4 bytes).
    last_block.insert(0, 0);

    let headBlockNumber = u32::from_be_bytes(last_block.try_into().unwrap());

    Ok(headBlockNumber)
}

async fn getBlockHash(blockNumber: u32) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let resp = client
        .post("https://rpc.polkadot.io")
        .json(&json! {

            {
                "id": 1,
                "jsonrpc": "2.0",
                "method": "chain_getBlockHash",
                "params": [blockNumber]
            }

        })
        .send()
        .await?;

    let text: Value = resp.json().await.unwrap();

    let hash = text["result"].as_str().unwrap();

    Ok(hash.to_string())
}

async fn getBlockData(blockHash: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let resp = client
        .post("https://rpc.polkadot.io")
        .json(&json! {

                 {
                     "id": 1,
                     "jsonrpc":"2.0",
                     "method":"chain_getBlock",
                     "params": [blockHash]
                 }

        })
        .send()
        .await?;

    let text: Value = resp.json().await.unwrap();

    println!("{:?}", text);

    Ok(text["result"]
        .as_str()
        .unwrap()
        .trim_start_matches("0x")
        .to_string())
}
