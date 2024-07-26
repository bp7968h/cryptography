# Cryptography
This repository is dedicated to exploring and implementing various cryptographic hash functions in Rust, starting with SHA-1. The primary goal of this project is educational, aimed at understanding the mechanisms and applications of hash functions in cryptography.

## Getting Started

### Prerequisites

Ensure you have Rust installed. If not, you can install it by following the instructions on the [official Rust website](https://www.rust-lang.org/learn/get-started).

### Installation

Clone the repository to begin exploring the hash functions:

```bash
git clone https://github.com/your-username/cryptography.git
cd cryptography
```

## Implemented Algorithms

### SHA-1
SHA-1 (Secure Hash Algorithm 1) is a cryptographic hash function which produces a 160-bit hash value known as a message digest. Although SHA-1 has been found to be vulnerable to certain types of attacks and is no longer recommended for security-critical cryptographic purposes, it remains widely used for general checksum purposes and in non-critical applications due to its speed.

#### Usage
To use the SHA-1 function in your Rust code, first ensure you have the library included in your `Cargo.toml`:
```
[dependencies]
cryptography = { git = "https://github.com/your-username/cryptography.git" }
```
Then, you can compute a SHA-1 hash as follows:

```
use cryptography::SHA1;

fn main() {
    let mut sha1 = SHA1::new();
    let result = sha1.hash("example");
    println!("SHA-1 hash: {}", result);
}
```

## `To be Continued, Any suggestions are welcomed`
