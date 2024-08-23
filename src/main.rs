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

    let flags = config.flags;
    let mut count = 0;

    match config.input_lines {
        pattern_processor::InputEnum::StdInput(line) => {
            for (index, line) in line.iter().enumerate() {
                if grep::grep(&flags, &line, &config.pattern) {
                    if !flags.invert_match {
                        count += 1;
                        if !flags.count {
                            if flags.line_numbers {
                                println!("{}:{line}", index + 1);
                            } else {
                                println!("{line}");
                            }
                        }
                    }
                } else if flags.invert_match {
                    count += 1;
                    if !flags.count {
                        if flags.line_numbers {
                            println!("{}:{line}", index + 1);
                        } else {
                            println!("{line}");
                        }
                    }
                }
            }
        }
        pattern_processor::InputEnum::FileInput(files) => {
            for file in files.iter() {
                for (index, line) in file.buffer.iter().enumerate() {
                    if grep::grep(&flags, &line, &config.pattern) {
                        if !flags.invert_match {
                            count += 1;
                            if !flags.count {
                                if flags.line_numbers {
                                    println!("{}:{}:{line}", file.name, index + 1);
                                } else {
                                    println!("{}:{line}", file.name);
                                }
                            }
                        }
                    } else if flags.invert_match {
                        count += 1;
                        if !flags.count {
                            if flags.line_numbers {
                                println!("{}:{}:{line}", file.name, index + 1);
                            } else {
                                println!("{}:{line}", file.name);
                            }
                        }
                    }
                }
            }
        }
    }
    if flags.count {
        println!("{count}");
    }
    process::exit(0)
}
