use std::fs;
use std::io::Error;

fn extract_errors(text: &str) -> Vec<&str> {
    let split = text.split('\n');
    let mut results = Vec::new();
    for line in split {
        if line.starts_with("ERROR") {
            results.push(line);
        }
    }

    results
}

fn main() -> Result<(), Error> {
    let text = fs::read_to_string("logs.txt")?;
    let errors = extract_errors(text.as_str());
    fs::write("errors.txt", errors.join("\n"))?;

    Ok(())

    // let text = fs::read_to_string("logs.txt")
    //     .expect("Failed to read logs.txt");
    // let errors = extract_errors(text.as_str());
    // fs::write("errors.txt", errors.join("\n"))
    //     .expect("Failed to write errors.txt");

    // match fs::read_to_string("logs.txt") {
    //     Ok(content) => {
    //         let errors = extract_errors(content.as_str());
    //         match fs::write("errors.txt", errors.join("\n")) {
    //             Ok(_) => {
    //                 println!("Errors extracted successfully to errors.txt");
    //             }
    //             Err(e) => {
    //                 eprintln!("Failed to write errors to file: {}", e);
    //             }
    //         }
    //     }
    //     Err(e) => {
    //         eprintln!("Failed to read file: {}", e);
    //     }
    // }
}
