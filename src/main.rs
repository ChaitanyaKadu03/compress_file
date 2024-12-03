use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs;
use std::io::Write;

fn main() {
    let file_path = "example.txt";
    let output_path = "copy.txt";

    let content = fs::read_to_string(file_path);

    match content {
        Ok(data) => {
            println!("File content:\n{}", data);

            let mut encoder = GzEncoder::new(Vec::new(), Compression::default());

            match encoder.write_all(data.as_bytes()) {
                Ok(_) => match encoder.finish() {
                    Ok(compressed_data) => match fs::write(output_path, compressed_data) {
                        Ok(_) => println!("File compressed successfully to {}", output_path),
                        Err(error) => println!("Error writing compressed file: {}", error),
                    },
                    Err(error) => {
                        println!("Error finalizing compression: {}", error)
                    }
                },
                Err(error) => {
                    println!("Error during compression: {}", error)
                }
            }
        }
        Err(err) => println!("Error reading file: {}", err),
    }
}
