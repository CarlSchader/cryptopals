use hex;

static INPUT1: &str = "1c0111001f010100061a024b53535009181c";
static INPUT2: &str = "686974207468652062756c6c277320657965";
static EXPECTED: &str = "746865206b696420646f6e277420706c6179";

fn fixed_xor(buf1: &[u8], buf2: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    if buf1.len() != buf2.len() {
        return Err("Buffers must be equal length".into());
    }

    let result = buf1.iter().zip(buf2.iter()).map(|(a, b)| a ^ b).collect();

    Ok(result)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Fixed XOR Challenge");
    println!("Input 1: {}", INPUT1);
    println!("Input 2: {}", INPUT2);

    let bytes1 = hex::decode(INPUT1)?;
    let bytes2 = hex::decode(INPUT2)?;

    let xor_result = fixed_xor(&bytes1, &bytes2)?;
    let result_hex = hex::encode(xor_result);

    println!("Result:  {}", result_hex);
    println!("Expected: {}", EXPECTED);

    if result_hex == EXPECTED {
        println!("✓ Success! XOR result matches expected output.");
    } else {
        println!("✗ Failed! XOR result does not match expected output.");
    }

    // Let's also decode the result to see what it says
    if let Ok(decoded_result) = hex::decode(&result_hex) {
        if let Ok(ascii_result) = String::from_utf8(decoded_result) {
            println!("Decoded result: '{}'", ascii_result);
        }
    }

    Ok(())
}
