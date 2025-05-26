use base64::prelude::*;
use hex;

static HEX_STRING: &str = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("hex: {HEX_STRING}");

    let bytes = hex::decode(HEX_STRING)?;

    let base_64_string = BASE64_STANDARD.encode(bytes);

    println!("base_64: {base_64_string}");

    Ok(())
}
