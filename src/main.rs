extern crate base64;
extern crate hex;

fn main() {
    challenge1();
    challenge2();
    challenge3();
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
    let letters = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];
    let mut top_score = 0;
    let mut top_value = String::new();

    for letter in letters.iter() {
        let value = String::from_utf8(single_xor(&hex_to_bytes(input), letter.clone())).unwrap();
        let score = score_string(&value);

        if score > top_score {
            top_score = score;
            top_value = value;
        }
    }

    println!("{:?}", top_value);
}

fn score_string(input: &String) -> i32 {
    let mut acc = 0;

    for i in input.chars() {
        match i {
            'E' => acc += 26,
            'T' => acc += 25,
            'A' => acc += 24,
            'O' => acc += 23,
            'I' => acc += 22,
            'N' => acc += 21,
            'S' => acc += 20,
            'H' => acc += 19,
            'R' => acc += 18,
            'D' => acc += 17,
            'L' => acc += 16,
            'C' => acc += 15,
            'U' => acc += 14,
            'M' => acc += 13,
            'W' => acc += 12,
            'F' => acc += 11,
            'G' => acc += 10,
            'Y' => acc += 9,
            'P' => acc += 8,
            'B' => acc += 7,
            'V' => acc += 6,
            'K' => acc += 5,
            'J' => acc += 4,
            'X' => acc += 3,
            'Q' => acc += 2,
            'Z' => acc += 1,
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
