extern crate eth_crypto;

use eth_crypto::prelude::*;

#[allow(dead_code)]
fn example1() {
    let mut a = [1, 2, 3, 4, 5];
    let hash1 = hash(&a);
    a[0] = 0;
    let hash2 = hash(&a);
    println!("hash-1: {:x}", &hash1);
    println!("hash-2: {:x}", &hash2);
}

fn main() {
    example1();
}
