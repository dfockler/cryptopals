extern crate base64;
extern crate hex;

use base64::{encode};
use hex::{decode};

fn main() {
    let x = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let output = hex_to_base64(x);
    println!("{:?}", output);
}

fn hex_to_base64(input: &str) -> String {
    encode(&decode(input).unwrap())
}
