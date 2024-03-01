# README 

# Description
- This program calculates the hash value of a specified file using the BLAKE3 cryptographic hash function. It aids in file integrity verification, security checks, automated workflows, and ensures compatibility across platforms. It is particularly useful for verifying the integrity of firmware and cryptographic signatures, keys, and certificates.

# Usage
- To use the program, run the executable specifying the path to the file for which you want to calculate the hash.

 ## For example: 
    1. cargo run <filename>
    2. Run cargo build --release to build the executable.
    3. openSSl Command Line Arguments:
      1. SHA-256:  openssl dgst -sha256 <filename>
      2. SHA-384:  openssl dgst -sha384 <filename>
      3. SHA-512:  openssl dgst -sha512 <filename>
      4. MD5:      openssl dgst -md5 <filename>


### The executable file will be located in the target/release/ directory.

# Dependencies
  - clap: Used for parsing command-line arguments.
  - blake3: Provides functionality for computing BLAKE3 hashes.
  - openssl: For additional cryptographic operations (not currently utilized in this version).