use std::env::{args, Args};

fn main() {
    // Read the amount of addresses that either sent some transaction or received some money on a blockchain.
    let mut a: Args = args();
    let start_block = match a.nth(1) {
        Some(x) => x,
        None => panic!("Start block is not provided!"),
    };
    let process_count = match a.nth(2) {
        Some(y) => y,
        None => panic!("Process count is not specified!"),
    };

    println!("{:?} {:?}", start_block, process_count);
}
