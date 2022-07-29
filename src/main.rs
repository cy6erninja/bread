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
    // let rpc = match a.nth(1) {
    //     Some(a) => a,
    //     None => panic!("RPC url is not provided."),
    // };

    //let start_block = match a.nth(2) {
    //    Some(x) => x,
    //    None => "1".to_string(),
    //};

    //let process_count = match a.nth(3) {
    //    Some(y) => y,
    //    None => "1".to_string(),
    //};

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

    let l = u32::from_be_bytes(last_block.try_into().unwrap());

    println!("{:?}", l);

    Ok(())
}
