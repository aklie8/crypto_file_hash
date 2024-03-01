/* Import necessary items from standard library and external crates */
use clap::{App, Arg};            // For parsing command-line arguments
use std::io::Read;                // For file input/output operations
use std::fs::File;                // For file operations
use std::path::Path;              // For working with file paths
use blake3::Hasher;                // For computing BLAKE3 hashes
use sha2::{Digest, Sha256, Sha384, Sha512};   // For SHA-256 and SHA-512 hashing
use md5;                          // For MD5 hashing
use blake2b_simd::Params as Blake2bParams; // For BLAKE2b hashing
use blake2s_simd::Params as Blake2sParams; // For BLAKE2s hashing
use bcrypt;                        // For Bcrypt hashing




/* Main function where the program execution starts */
fn main() {
    // Define the command-line interface using Clap
    let matches = App::new("Calculate File Hash")    // Set the name of the program
        .version("1.0")                               // Set the version number
        .author("Aklile Tesfaye <astesfaye@dons.usfca.edu>")    // Set the author information
        .about("Helps with file integrity, security checks, automated workflows & ensures compatibility across platforms. Vital for verifying the integrity of firmware and cryptographic signatures/keys/certificates.")    // Provide a brief description of the program
        .arg(Arg::with_name("filename")    // Define a command-line argument named "filename"
            .help("Sets the input file to calculate hash for")    // Provide a help message for this argument
            .index(1))    // Set the position of this argument in the command-line
        .get_matches();    // Parse the command-line arguments

    // Extract the value of the "filename" argument provided by the user
    let filename = matches.value_of("filename").unwrap();

    // Create a Path object from the filename
    let path = Path::new(filename);

    // Open the file specified by the user
    let mut file = match File::open(&path) {
        Err(why) => {    // Handle the case where file opening fails
            eprintln!("Error: Couldn't open {}: {}", path.display(), why);
            std::process::exit(1);    // Exit the program with an error code
        }
        Ok(file) => file,    // Continue with the opened file if successful
    };

    // Create a buffer to hold the contents of the file
    let mut buffer = Vec::new();

    // Read the contents of the file into the buffer
    if let Err(why) = file.read_to_end(&mut buffer) {
        // Handle the case where file reading fails
        eprintln!("Error: Couldn't read {}: {}", path.display(), why);
        std::process::exit(1);    // Exit the program with an error code
    }

    // Calculate the BLAKE3 hash of the file contents
    let mut hasher_blake3 = Hasher::new();
    hasher_blake3.update(&buffer);
    let hash_blake3 = hasher_blake3.finalize();

    // Calculate the SHA-256 hash of the file contents
    let hash_sha256 = Sha256::digest(&buffer);

    // Calculate the SHA-384 hash of the file contents
    let hash_sha384 = Sha384::digest(&buffer);


    // Calculate the SHA-512 hash of the file contents
    let hash_sha512 = Sha512::digest(&buffer);

    // Calculate the MD5 hash of the file contents
    let hash_md5 = md5::compute(&buffer);

    // Calculate the BLAKE2b hash of the file contents
    let hash_blake2b = Blake2bParams::new().hash_length(64).hash(&buffer);

    // Calculate the Bcrypt hash of the file contents
    let bcrypt_hash = bcrypt::hash(&buffer, bcrypt::DEFAULT_COST).unwrap();

     // Calculate the BLAKE2s hash of the file contents
     let hash_blake2s = Blake2sParams::new().hash_length(32).hash(&buffer);


    // Print the calculated hash as a hexadecimal string
    println!("BLAKE3 Hash:\n  {:?}\n", hash_blake3);
    println!("SHA-256 Hash:\n {:?}\n", hash_sha256);
    println!("SHA-384 Hash:\n {:?}\n", hash_sha384);
    println!("SHA-512 Hash:\n {:?}\n", hash_sha512);
    println!("MD5 Hash:\n {:?}\n", hash_md5);
    println!("BLAKE2b Hash:\n {:?}\n", hash_blake2b);
    println!("Bcrypt Hash:\n {}\n", bcrypt_hash);
    println!("BLAKE2s Hash:\n {:?}\n", hash_blake2s);




    //to make the hash value into hexadecimal format 
    //println!("{:x?}", hash.as_bytes());

}