extern crate base64;
extern crate hex;

use std::fs::File;
use std::io::prelude::*;

fn main() {
    challenge1();
    challenge2();
    challenge3();
    challenge4();
}

fn challenge1() {
    let x = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let output = hex_to_base64(x);
    
    println!("{:?}", output);
}

fn challenge2() {
    let x = "1c0111001f010100061a024b53535009181c";
    let y = "686974207468652062756c6c277320657965";

    let output = xor(&hex_to_bytes(x), &hex_to_bytes(y));

    println!("{:?}", output);
}

fn challenge3() {
    let input = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let top_value = top_scored_value(&input);

    println!("{}", top_value);
}

fn challenge4() {
    let mut file = File::open("4.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    for line in contents.lines() {
        let value = top_scored_value(&line);
        println!("{}", value);
    }
}

// Look at endianness, all the output characters are reverse capitalized
fn top_scored_value(input: &str) -> String {
    let letters = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'];
    let mut top_score = 0;
    let mut top_value = String::new();

    for letter in letters.iter() {
        let value = unsafe {
            String::from_utf8_unchecked(single_xor(&hex_to_bytes(input), letter.clone()))
        };
        let score = score_string(&value);

        if score > top_score {
            top_score = score;
            top_value = value;
        }
    }

    top_value
}

fn score_string(input: &String) -> i32 {
    let mut acc = 0;

    for i in input.chars() {
        match i {
            'E' | 'e' => acc += 26,
            'T' | 't' => acc += 25,
            'A' | 'a' => acc += 24,
            'O' | 'o' => acc += 23,
            'I' | 'i' => acc += 22,
            'N' | 'n' => acc += 21,
            'S' | 's' => acc += 20,
            'H' | 'h' => acc += 19,
            'R' | 'r' => acc += 18,
            'D' | 'd' => acc += 17,
            'L' | 'l' => acc += 16,
            'C' | 'c' => acc += 15,
            'U' | 'u' => acc += 14,
            'M' | 'm' => acc += 13,
            'W' | 'w' => acc += 12,
            'F' | 'f' => acc += 11,
            'G' | 'g' => acc += 10,
            'Y' | 'y' => acc += 9,
            'P' | 'p' => acc += 8,
            'B' | 'b' => acc += 7,
            'V' | 'v' => acc += 6,
            'K' | 'k' => acc += 5,
            'J' | 'j' => acc += 4,
            'X' | 'x' => acc += 3,
            'Q' | 'q' => acc += 2,
            'Z' | 'z' => acc += 1,
            _ => acc += 0,
        }
    }

    acc
}

fn hex_to_bytes(input: &str) -> Vec<u8> {
    hex::decode(input).unwrap()
}

fn hex_to_base64(input: &str) -> String {
    base64::encode(&hex_to_bytes(input))
}

fn xor(a: &Vec<u8>, b: &Vec<u8>) -> String {
    let mut output: Vec<u8> = Vec::new();

    for (a, b) in a.iter().zip(b) {
        output.push(a ^ b);
    }

    hex::encode(&output)
}

fn single_xor(input: &Vec<u8>, value: char) -> Vec<u8> {
    let mut output: Vec<u8> = Vec::new();

    for byte in input.iter() {
        output.push(byte ^ value as u8);
    }

    output
}
