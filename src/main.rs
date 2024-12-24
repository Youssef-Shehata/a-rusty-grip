mod grep;
mod pattern_processor;
use std::{env, process};
#[derive(Debug)]
struct Matches<'a> {
    line: &'a str,
    line_number: usize,
    file_name: Option<String>,
}
impl<'a> Matches<'a> {
    fn new(line: &'a str, file_name: Option<String>, line_number: usize) -> Matches<'a> {
        Self {
            line,
            line_number,
            file_name,
        }
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let config = pattern_processor::Config::new(&args).unwrap_or_else(|e| {
        eprintln!("ERROR : {}", e);
        process::exit(1)
    });

    let flags = config.flags;
    let mut matches: Vec<Matches> = Vec::new();

    for input_buffer in config.input_lines.iter() {
        for (line_number, line) in input_buffer.content.iter().enumerate() {
            let matched = grep::grep(&flags, &line, &config.pattern);

            if matched && !flags.invert_match {
                matches.push(Matches::new(
                    line,
                    input_buffer.file_name.clone(),
                    line_number,
                ));
            } else if !matched && flags.invert_match {
                matches.push(Matches::new(
                    line,
                    input_buffer.file_name.clone(),
                    line_number,
                ));
            }
        }
    }

    if flags.count {
        println!("  {}", matches.len());
    } else {
        write_output(flags, matches);
    }

    process::exit(0)
}
fn write_output(flags: pattern_processor::Flags, matches: Vec<Matches>) {
    for matchy in matches.iter() {
        let output;
        match &matchy.file_name {
            Some(f) => {
                if flags.line_numbers {
                    output = format!("  {}:{}:{}", f, matchy.line_number, matchy.line);
                } else {
                    output = format!("  {}:{}", f, matchy.line);
                }
            }
            None => {
                if flags.line_numbers {
                    output = format!("  {}:{}", matchy.line_number, matchy.line);
                } else {
                    output = format!("  {}", matchy.line);
                }
            }
        }
        println!("{}", output);
    }
}
