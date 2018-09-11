extern crate base64;
extern crate hex;

use std::fs::File;
use std::io::prelude::*;

fn main() {
    // challenge1();
    // challenge2();
    // challenge3();
    // challenge4();
    // challenge5();
    // challenge6();
    challenge7();
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
    let (_, top_value, _) = top_scored_value(&hex::decode(input.as_bytes()).unwrap());

    println!("{}", top_value);
}

fn challenge4() {
    let mut file = File::open("4.txt").unwrap();
    let mut contents = String::new();
    let mut top_score = 0;
    let mut top_value = String::new();
    file.read_to_string(&mut contents).unwrap();

    for line in contents.lines() {
        let (score, value, _) = top_scored_value(&hex::decode(line.as_bytes()).unwrap());
        if score > top_score {
            top_score = score;
            top_value = value;
        }
    }

    println!("{}", top_value);
}

fn challenge5() {
    let plaintext = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
    let key = "ICE";
    let output = hex::encode(repeating_xor(plaintext, key));

    println!("{}", output);
}

fn challenge6() {
    let mut file = File::open("6.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let stripped = contents.replace("\n", "");
    let decoded = base64::decode(&stripped).unwrap();
    let mut smallest = 320f32;
    let mut smallest_keysize = 40;

    for keysize in 2..40 {
        let mut chunks = decoded.chunks(keysize);
        let mut average = 0;

        // This is not the best solution, but it works for this scenario
        for _ in 0..20 {
            average += hamming_distance(chunks.next().unwrap(), chunks.next().unwrap());
        }
        
        let normal = average as f32 / keysize as f32;

        if normal < smallest as f32 {
            smallest = normal;
            smallest_keysize = keysize;
        }
    }

    let mut keygen: Vec<u8> = Vec::new();
    for index in 0..smallest_keysize {
        let mut gen = Vec::new();
        for (i, byte) in decoded.bytes().enumerate() {
            if i % smallest_keysize == index {
                gen.push(byte.unwrap());
            }
        }

        let (_, _, top_byte) = top_scored_value(&gen);
        keygen.push(top_byte);
    }

    let key = String::from_utf8(keygen).unwrap();
    println!("{}\n", key);
    let output = repeating_xor_decode(&decoded, &key);
    println!("{}", output);
}

fn challenge7() {
    // 1001-1010
    // 0011-0101 35
    // 
    let value = generate_aes_sbox();
    println!("{}", hex::encode([0x13, 0xff, 0xa8, 0x40]));
    let check = sub_word(&[0x13, 0xff, 0xa8, 0x40], &value);

    println!("{}", hex::encode(check));
    // eca(240, 46);
}

fn rotl8(x: u8, shift: u8) -> u8 {
    x << shift | x >> (8 - shift)
}

// Based on code from here
// https://crypto.stackexchange.com/questions/12956/multiplicative-inverse-in-operatornamegf28/12962#12962
fn generate_aes_sbox() -> [u8; 256] {
    let (mut p, mut q) = (1u8, 1u8);
    let mut sbox = [0u8; 256];

    loop {
        p = p ^ (p << 1) ^ if p & 0x80 > 0 { 0x1B } else { 0 };

        q ^= q << 1;
        q ^= q << 2;
        q ^= q << 4;
        q ^= if q & 0x80 > 0 { 0x09 } else { 0 };

        let xformed = q ^ rotl8(q, 1) ^ rotl8(q, 2) ^ rotl8(q, 3) ^ rotl8(q, 4);

        sbox[p as usize] = xformed ^ 0x63;

        if p == 1 { break; }
    }

    sbox[0] = 0x63;

    sbox
}

// Substitutes input values from the sbox
fn sub_word(inputs: &[u8; 4], sbox: &[u8; 256]) -> [u8; 4] {
    let mut output = [0u8; 4];

    for (i, value) in inputs.iter().enumerate() {
        let row = (value & 0xf0).wrapping_shr(4) * 16;
        let column = value & 0x0f;
        let index = (column + row) as usize;
        output[i] = sbox[index];
    }

    output
}

fn rcon(index: usize) -> u8 {
    let rcon_values = [0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80, 0x1B, 0x36];
    rcon_values[index - 1]
}

fn top_scored_value(input: &[u8]) -> (i32, String, u8) {
    let mut top_score = 0;
    let mut top_value = String::new();
    let mut top_byte = 0;

    for byte in 0..255 {
        let value = match String::from_utf8(single_xor(input, byte)){
            Ok(n) => n,
            Err(_) => continue,
        };

        let score = score_string(&value);

        if score > top_score {
            top_score = score;
            top_value = value;
            top_byte = byte;
        }
    }

    (top_score, top_value, top_byte)
}

fn score_string(input: &String) -> i32 {
    let mut acc = 0;

    for i in input.chars() {
        match i {
            'e' => acc += 26,
            't' => acc += 25,
            'a' => acc += 24,
            'o' => acc += 23,
            'i' => acc += 22,
            'n' => acc += 21,
            's' => acc += 20,
            'h' => acc += 19,
            'r' => acc += 18,
            'd' => acc += 17,
            'l' => acc += 16,
            'c' => acc += 15,
            'u' => acc += 14,
            'm' => acc += 13,
            'w' => acc += 12,
            'f' => acc += 11,
            'g' => acc += 10,
            'y' => acc += 9,
            'p' => acc += 8,
            'b' => acc += 7,
            'v' => acc += 6,
            'k' => acc += 5,
            'j' => acc += 4,
            'x' => acc += 3,
            'q' => acc += 2,
            'z' => acc += 1,
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

fn single_xor(input: &[u8], value: u8) -> Vec<u8> {
    let mut output: Vec<u8> = Vec::new();

    for byte in input.iter() {
        output.push(byte ^ value);
    }

    output
}

fn repeating_xor(input: &str, key: &str) -> Vec<u8> {
    let mut output: Vec<u8> = Vec::new();
    let mut key_offset = 0;

    for byte in input.as_bytes() {
        output.push(byte ^ key.as_bytes()[key_offset % key.len()]);
        key_offset += 1;
    }

    output
}

fn repeating_xor_decode(input: &[u8], key: &str) -> String {
    let mut output: Vec<u8> = Vec::new();
    let mut key_offset = 0;

    for byte in input.iter() {
        output.push(byte ^ key.as_bytes()[key_offset % key.len()]);
        key_offset += 1;
    }

    String::from_utf8(output).unwrap()
}

fn hamming_distance(a: &[u8], b: &[u8]) -> u32 {
    let mut distance = 0;

    for (first, second) in a.bytes().zip(b.bytes()) {
        let diff = first.unwrap() ^ second.unwrap();
        distance += diff.count_ones();
    }
    distance
}
