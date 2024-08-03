use std::env;
use std::io;
use std::process;

fn match_pattern(input_line: &str, pattern: &str) -> bool {
    match pattern {
        single_letter if single_letter.len() == 1 => input_line.contains(single_letter),
        r#"\w"# => input_line.contains(|c: char| c.is_alphanumeric()),
        r#"\d"# => input_line.contains(|c: char| c.is_digit(10)),
        pat if pat.starts_with("[^") && pat.ends_with("]") => pattern[2..pattern.len() - 1]
            .chars()
            .all(|x| !input_line.contains(x)),
        pat if pat.starts_with("[") && pat.ends_with("]") => {
            let pattern = &pattern[1..pattern.len() - 1];
            for c in pattern.chars() {
                if input_line.contains(c) {
                    return true;
                }
            }
            return false;
        }
        _ => false,
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
