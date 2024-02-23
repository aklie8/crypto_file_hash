// Import necessary items from standard library
use clap::{App, Arg};  // For parsing command-line arguments
use std::fs::File;      // For file operations
use std::io::{self, Read};  // For input/output operations
use std::path::Path;    // For working with file paths
use sha2::{Digest, Sha256};  // For computing SHA-256 hashes

// Main function where the program execution starts
fn main() {
    // Define the command-line interface using Clap
    let matches = App::new("Calculate File Hash")  // Set the name of the program
        .version("1.0")  // Set the version number
        .author("Aklile Tesfaye <astesfaye@dons.usfca.edu>")  // Set the author information
        .about("Helps with file integrity, security checks, automated workflows & ensures compatibility across platforms. Vital for verifying the integrity of firmware and cryptographic signatures/keys/certificates.")  // Provide a brief description of the program
        .arg(Arg::with_name("filename")  // Define a command-line argument named "filename"
            .help("Sets the input file to calculate hash for")  // Provide a help message for this argument
            .required(true)  // Specify that this argument is required
            .index(1))  // Set the position of this argument in the command-line
        .get_matches();  // Parse the command-line arguments

    // Extract the value of the "filename" argument provided by the user
    let filename = matches.value_of("filename").unwrap();

    // Create a Path object from the filename
    let path = Path::new(filename);

    // Open the file specified by the user
    let mut file = match File::open(&path) {
        Err(why) => {  // Handle the case where file opening fails
            eprintln!("Error: Couldn't open {}: {}", path.display(), why);
            std::process::exit(1);  // Exit the program with an error code
        }
        Ok(file) => file,  // Continue with the opened file if successful
    };

}
