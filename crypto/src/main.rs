use crypto::digest::Digest;
use crypto::sha2::Sha256;

fn main() {
    // Input data
    let data = "Hello, World!";

    // Create a SHA-256 hash object
    let mut sha256 = Sha256::new();

    // Update the hash with the data
    sha256.input_str(data);

    // Get the resulting hash as a hex string
    let result = sha256.result_str();

    // Print the result
    println!("SHA-256 hash: {}", result);
}
