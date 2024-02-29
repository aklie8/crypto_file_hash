/* Import necessary items from standard library and external crates */
use clap::{App, Arg};            // For parsing command-line arguments
use std::io::Read;                // For file input/output operations
use std::fs::File;                // For file operations
use std::path::Path;              // For working with file paths
use blake3::Hasher;                // For computing BLAKE3 hashes

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
    let mut hasher = Hasher::new();
    hasher.update(&buffer);
    let hash = hasher.finalize();

    // Print the calculated hash as a hexadecimal string
    println!("Hash value of test.txt: {:?}", hash);

    //to make the hash value into hexadecimal format 
    //println!("{:x?}", hash.as_bytes());

}
