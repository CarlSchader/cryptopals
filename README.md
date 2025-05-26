# Cryptopals Challenges

A Rust implementation of the [Cryptopals Crypto Challenges](https://cryptopals.com/) - a collection of practical cryptography exercises designed to teach real-world cryptographic attacks and implementations.

## About Cryptopals

The Cryptopals challenges are a series of applied cryptography exercises that teach you to break real-world crypto. The challenges are organized into sets, each focusing on different aspects of cryptography:

- **Set 1**: Basics (Base64, XOR, frequency analysis)
- **Set 2**: Block crypto (AES, CBC, padding oracles)
- **Set 3**: Block & stream crypto (CBC bitflipping, MT19937)
- **Set 4**: Stream crypto and randomness (SHA-1, MD4, timing attacks)
- **Set 5**: Diffie-Hellman and friends
- **Set 6**: RSA and DSA
- **Set 7**: Hashes and MACs
- **Set 8**: Abstract Algebra

## Current Implementation Status

### Set 1: The Basics
- [x] **Challenge 1**: Convert hex to base64
- [ ] Challenge 2: Fixed XOR
- [ ] Challenge 3: Single-byte XOR cipher
- [ ] Challenge 4: Detect single-character XOR
- [ ] Challenge 5: Implement repeating-key XOR
- [ ] Challenge 6: Break repeating-key XOR
- [ ] Challenge 7: AES in ECB mode
- [ ] Challenge 8: Detect AES in ECB mode

### Set 2-8
- [ ] Coming soon...

## Prerequisites

- [Rust](https://rustup.rs/) (edition 2024)
- Cargo (comes with Rust)

## Installation

```bash
git clone <your-repo-url>
cd cryptopals
```

## Usage

Each challenge is implemented as a separate binary in the `src/bin/` directory. To run a specific challenge:

```bash
# Run Challenge 1 (hex to base64)
cargo run --bin p1
```

To run all tests:

```bash
cargo test
```

## Project Structure

```
cryptopals/
├── src/
│   └── bin/
│       └── p1.rs          # Challenge 1: Convert hex to base64
├── Cargo.toml             # Project dependencies and metadata
└── README.md              # This file
```

## Dependencies

- [`base64`](https://crates.io/crates/base64) - Base64 encoding/decoding
- [`hex`](https://crates.io/crates/hex) - Hexadecimal encoding/decoding

## Challenge Solutions

### Challenge 1: Convert hex to base64

Converts a hex string to base64. This challenge introduces the fundamental data representations used throughout cryptography.

**Input**: `49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d`

**Output**: `SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t`

**Decoded message**: "I'm killing your brain like a poisonous mushroom"

## Development

### Adding New Challenges

1. Create a new file in `src/bin/` named `p{number}.rs`
2. Implement the challenge solution
3. Add any necessary dependencies to `Cargo.toml`
4. Update this README with the challenge status

### Code Style

This project follows standard Rust conventions:
- Use `cargo fmt` for formatting
- Use `cargo clippy` for linting
- Write tests for your implementations
- Document public APIs

## Learning Resources

- [Cryptopals Official Site](https://cryptopals.com/)
- [The Rust Programming Language Book](https://doc.rust-lang.org/book/)
- [Rust Cryptography Ecosystem](https://github.com/RustCrypto)

## Contributing

This is a personal learning project, but suggestions and improvements are welcome! Please feel free to:

- Point out bugs or issues
- Suggest more efficient implementations
- Share alternative approaches

## License

This project is for educational purposes. The Cryptopals challenges are created by Matasano Security (now NCC Group).

## Disclaimer

These implementations are for educational purposes only and should not be used in production systems. The goal is to learn about cryptographic attacks and implementations, not to create secure cryptographic libraries.