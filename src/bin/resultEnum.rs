// Import the `read_to_string` function from the Rust standard library's `fs` module
use std::fs::read_to_string;

fn main() {
    // Call `read_from_fileComputer` with a file path and store the result in `ans`
    let ans = read_from_fileComputer(String::from("a.txt"));
    // `ans` will contain either Ok(String) if the file is read successfully,
    // or Err(String) if the file read fails.
}

// Function to read a file and return either its contents (Ok) or an error message (Err)
fn read_from_fileComputer(file_path: String) -> Result<String, String> {
    // Attempt to read the file into a string
    let result = read_to_string(file_path);  // This returns a Result type

    // Match on the result of `read_to_string`
    match result {
        // If successful (Ok), return the file contents
        Ok(data) => Ok(data),
        // If an error occurs (Err), return an error message
        Err(err) => Err(String::from("File not read")),
    }
}

