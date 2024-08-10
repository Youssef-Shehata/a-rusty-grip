use grep_starter_rust::*;
use std::env;

use std::process;
// Usage: echo <input_text> | your_program.sh -E <pattern>
fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");
    let args: Vec<String> = env::args().collect();

    if args[1] != "-E" {
        println!("Expected first argument to be '-E'");
        process::exit(1);
    }

    let config = Config::new(&args).unwrap_or_else(|e| {
        eprintln!("error parsing input : {}", e);
        process::exit(1)
    });

    if grep(&config.input_line, &config.pattern) {
        println!("match");
        process::exit(0)
    }

    println!("failed");
    process::exit(1)
}
