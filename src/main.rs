use std::fs;

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

fn main() {
    match fs::read_to_string("logs.txt") {
        Ok(content) => {
            let errors = extract_errors(content.as_str());
            println!("{:#?}", errors);
        }
        Err(e) => {
            eprintln!("Failed to read file: {}", e);
        }
    }

    
}
