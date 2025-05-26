use hex;
use std::collections::HashMap;

// The hex-encoded string that has been XOR'd against a single character
static ENCODED_HEX: &str = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";

// Function to score a piece of text based on character frequency in English
fn score_text(text: &[u8]) -> f64 {
    // English letter frequency (including space)
    let frequency: HashMap<char, f64> = [
        (' ', 0.18), ('e', 0.12), ('t', 0.09), ('a', 0.08), ('o', 0.07),
        ('i', 0.07), ('n', 0.07), ('s', 0.06), ('h', 0.06), ('r', 0.06),
        ('d', 0.04), ('l', 0.04), ('u', 0.03), ('c', 0.03), ('m', 0.02),
        ('w', 0.02), ('f', 0.02), ('g', 0.02), ('y', 0.02), ('p', 0.02),
        ('b', 0.01), ('v', 0.01), ('k', 0.01), ('j', 0.001), ('x', 0.001),
        ('q', 0.001), ('z', 0.001)
    ].iter().cloned().collect();

    let mut score = 0.0;
    let mut valid_chars = 0;

    for &byte in text {
        // Check if it's a printable ASCII character
        if byte >= 32 && byte <= 126 {
            valid_chars += 1;
            
            // Convert to lowercase for scoring
            let c = (byte as char).to_ascii_lowercase();
            if let Some(&freq) = frequency.get(&c) {
                score += freq;
            }
        } else if byte != b'\n' && byte != b'\r' && byte != b'\t' {
            // Penalize non-printable characters (except common whitespace)
            score -= 0.5;
        }
    }

    // Normalize by text length and boost score for texts with high percentage of valid chars
    if text.len() > 0 {
        score *= (valid_chars as f64 / text.len() as f64)
    }

    score
}

// Function to XOR a buffer with a single byte
fn single_byte_xor(buffer: &[u8], key: u8) -> Vec<u8> {
    buffer.iter().map(|&b| b ^ key).collect()
}

// Function to find the most likely key and decoded message
fn break_single_byte_xor(ciphertext: &[u8]) -> (u8, Vec<u8>, f64) {
    let mut best_score = std::f64::NEG_INFINITY;
    let mut best_key = 0;
    let mut best_plaintext = Vec::new();

    // Try every possible single byte key (0-255)
    for key in 0..=255 {
        let plaintext = single_byte_xor(ciphertext, key);
        let score = score_text(&plaintext);

        if score > best_score {
            best_score = score;
            best_key = key;
            best_plaintext = plaintext;
        }
    }

    (best_key, best_plaintext, best_score)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Single-byte XOR Challenge");
    println!("Encoded hex: {}", ENCODED_HEX);

    // Decode the hex string to bytes
    let ciphertext = hex::decode(ENCODED_HEX)?;
    
    // Break the cipher
    let (key, plaintext, score) = break_single_byte_xor(&ciphertext);
    
    // Convert plaintext bytes to a string
    let message = String::from_utf8_lossy(&plaintext);
    
    println!("Found key: '{}' (ASCII: {})", key as char, key);
    println!("Message: \"{}\"", message);
    println!("Score: {:.4}", score);

    Ok(())
}
