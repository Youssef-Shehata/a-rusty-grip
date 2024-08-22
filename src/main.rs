use std::env;

mod grep;
mod pattern_processor;
use std::process;
fn main() {
    let args: Vec<String> = env::args().collect();
    let config = pattern_processor::Config::new(&args).unwrap_or_else(|e| {
        eprintln!("ERROR : {}", e);
        process::exit(1)
    });

    for line in config.input_lines.iter() {
        if grep::grep(&line, &config.pattern) {
            println!("{line}");
        }
    }
    process::exit(0)
}
