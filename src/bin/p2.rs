use hex;

fn xor_buffers(buff1: &[u8], buff2: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    if buff1.len() != buff2.len() {
        return Err("Buffers must be of equal length".into());
    }

    Ok(buff1
        .iter()
        .zip(buff2.iter())
        .map(|(&a, &b)| a ^ b)
        .collect())
}

fn main() {
    let buffer1 = hex::decode("1c0111001f010100061a024b53535009181c").unwrap();
    let buffer2 = hex::decode("686974207468652062756c6c277320657965").unwrap();

    match xor_buffers(&buffer1, &buffer2) {
        Ok(result) => println!("XOR HEX result: {:?}", hex::encode(result)),
        Err(err) => println!("Error: {:?}", err),
    }
}
