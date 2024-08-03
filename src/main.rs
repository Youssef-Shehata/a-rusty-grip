use std::env;
use std::io;
use std::process;

fn match_pattern(input_line: &str, pattern: &str) -> bool {
    if pattern.starts_with("[") {
        let pattern = pattern.strip_prefix("[").unwrap();
        let pattern = pattern.strip_suffix("]").unwrap();

        for i in pattern.chars() {
            if !input_line.contains(i) {
                return false;
            }
        }
        return true;
    }
    if pattern.starts_with("\\") {
        match pattern {
            "\\d" => {
                return input_line.contains(|c: char| c.is_digit(10));
            }
            "\\w" => {
                return input_line.contains(|c: char| c.is_alphanumeric());
            }
            _ => {}
        };
    };
    if pattern.chars().count() == 1 {
        return input_line.contains(pattern);
    } else {
        panic!("Unhandled pattern: {}", pattern)
    }
}

// Usage: echo <input_text> | your_program.sh -E <pattern>
fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    if env::args().nth(1).unwrap() != "-E" {
        println!("Expected first argument to be '-E'");
        process::exit(1);
    }

    let pattern = env::args().nth(2).unwrap();
    let mut input_line = String::new();

    io::stdin().read_line(&mut input_line).unwrap();

    // Uncomment this block to pass the first stage
    if match_pattern(&input_line, &pattern) {
        println!("oioi");
        process::exit(0)
    } else {
        println!("nononon");
        process::exit(1)
    }
}
