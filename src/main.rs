use std::fs;

fn extract_errors(text: &str) -> Vec<String> {
    let split = text.split('\n');
    let mut results = Vec::new();
    for line in split {
        if line.starts_with("ERROR") {
            results.push(line.to_string());
        }
    }

    results
}

fn main() {
    let mut errors = Vec::new();

    match fs::read_to_string("logs.txt") {
        Ok(content) => {
            errors = extract_errors(content.as_str());

        }
        Err(e) => {
            eprintln!("Failed to read file: {}", e);
        }
    }

    println!("{:#?}", errors);
}
