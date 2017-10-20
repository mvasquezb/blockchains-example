extern crate eth_crypto;

use eth_crypto::prelude::*;
use eth_crypto::hash::keccak256::Hash;

#[allow(dead_code)]
fn example1() {
    let mut a = [1, 2, 3, 4, 5];
    let hash1 = hash(&a);
    a[0] = 0;
    let hash2 = hash(&a);
    println!("hash-1: {:x}", &hash1);
    println!("hash-2: {:x}", &hash2);
}

fn verify(password: &str, challenge: &str, response: &Hash) -> bool {
    let expected = hash_many(&[password, challenge]);
    &expected == response
}

fn example2() {
    let password = "decentralize all things";
    let challenge = " where appropiate";
    let response = hash_many(&[password, challenge]);
    if verify(password, challenge, &response) {
        println!("Welcome !");
    } else {
        println!("Access denied !");
    }
}

fn main() {
    // example1();
    example2();
}
