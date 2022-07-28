extern crate byteorder;
extern crate hex;
extern crate reqwest;
extern crate serde_json;
extern crate tokio;
use byteorder::{BigEndian, ReadBytesExt};
use serde_json::{json, Value};
use std::env::{args, Args};
use std::io::Cursor;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read the amount of addresses that either sent some transaction or received some money on a blockchain.
    let mut a: Args = args();
    // let rpc = match a.nth(1) {
    //     Some(a) => a,
    //     None => panic!("RPC url is not provided."),
    // };

    let start_block = match a.nth(2) {
        Some(x) => x,
        None => "1".to_string(),
    };

    let process_count = match a.nth(3) {
        Some(y) => y,
        None => "1".to_string(),
    };

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

    println!("{:?}", text["result"]["block"]["header"]["number"]);
    let mut last_block = hex::decode(
        text["result"]["block"]["header"]["number"]
            .as_str()
            .unwrap()
            .trim_start_matches("0x"),
    )
    .unwrap();

    //  let mut rdr = Cursor::new();
    //  let num = rdr.read_u32::<BigEndian>().unwrap();

    println!("{:?}", num);
    let l = u32::from_le_bytes(last_block[0..4].try_into().unwrap());

    println!("{:?}", l);

    Ok(())
}
