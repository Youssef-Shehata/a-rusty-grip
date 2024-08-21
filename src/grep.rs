use crate::pattern_processor::Config;
#[allow(unused)]
pub fn grep(input: &str, pattern: &str) -> bool {
    let pattern = Config::get_pattern(pattern);
    for pat in pattern.iter() {
        if match_pattern(input, pat) {
            return true;
        }
    }
    return false;
}

fn match_pattern(input: &str, pattern: &Vec<String>) -> bool {
    let mut i = 0;
    while i < input.len() {
        let mut matches = 0;
        for pat in pattern.iter() {
            let matched_pat;
            if pat.ends_with("+") || pat.ends_with("?") || pat.ends_with("*") {
                matched_pat = match_quantifier(input, pat, &mut i);
            } else if pat.starts_with("[") {
                matched_pat = match_group(input.chars().nth(i), pat, &mut i);
            } else if pat.starts_with("\\") {
                matched_pat = match_symbol(input.chars().nth(i), pat, &mut i);
            } else if pat == "." {
                matched_pat = match_wild_card(input.chars().nth(i).unwrap(), &mut i);
            } else {
                matched_pat = match_exact(input.chars().nth(i), pat, &mut i);
            }
            if matched_pat == false {
                break;
            }
            matches += 1;
        }
        if matches == pattern.len() {
            return true;
        }
    }
    return false;
}
fn match_exact(input: Option<char>, pattern: &str, i: &mut usize) -> bool {
    *i += 1;
    match input {
        Some(ch) => {
            println!("matching {} with exactly {}", ch, pattern);
            if &ch.to_string() == pattern {
                return true;
            } else {
                return false;
            }
        }
        None => false,
    }
}
fn line_matches(input: &str, pattern: &str, end: bool, i: &mut usize) -> bool {
    *i += 1;
    let mut input: Vec<&str> = input.split("").collect();
    let mut pattern: Vec<&str> = pattern.split("").collect();
    if end {
        input.reverse();
        pattern.reverse();
    }
    let input = input.join("");
    let pattern = pattern.join("");

    println!("matchin line {input} with {pattern}");
    for (mut i, ch) in pattern[1..].chars().enumerate() {
        if let Some(x) = input.chars().nth(i) {
            if x != ch {
                if end {
                    i = input.len() - i;
                }
                return false;
            }
        } else {
            if end {
                i = input.len() - i;
            }
            return false;
        }
    }
    return true;
}
fn match_group(input_option: Option<char>, pattern: &str, i: &mut usize) -> bool {
    *i += 1;

    match input_option {
        Some(input) => {
            println!("matching {} with group {}", input, pattern);

            if pattern.starts_with("[^") {
                let pattern = &pattern[2..pattern.len() - 1];
                if pattern.contains(input) {
                    return false;
                };
                return true;
            } else {
                let pattern = &pattern[1..pattern.len() - 1];
                if pattern.contains(input) {
                    return true;
                };
                return false;
            }
        }
        None => false,
    }
}
fn match_symbol(input_option: Option<char>, pattern: &str, i: &mut usize) -> bool {
    println!("matching symbol {}", pattern);

    *i += 1;
    match input_option {
        Some(input) => match pattern {
            r#"\w"# => {
                if input.is_alphanumeric() {
                    return true;
                }
                false
            }
            r#"\d"# => {
                if input.is_digit(10) {
                    return true;
                }
                false
            }
            _ => false,
        },
        None => false,
    }
}
fn match_quantifier(input: &str, pattern: &str, i: &mut usize) -> bool {
    println!("QUANTIFIER");
    match pattern {
        pat if pat.ends_with("+") => {
            let mut counter = 0;
            while *i < input.len() {
                if let Some(ch) = input.chars().nth(*i) {
                    if match_pattern(&ch.to_string(), &vec![pat[..pat.len() - 1].to_string()]) {
                        counter += 1;
                    } else {
                        break;
                    }
                } else {
                    break;
                }
                *i += 1;
            }
            if counter == 0 {
                return false;
            }
            return true;
        }

        pat if pat.ends_with("?") => {
            let mut iterator = 0;
            while iterator < 1 {
                if *i == input.len() - 1 {
                    break;
                }
                if let Some(ch) = input.chars().nth(*i) {
                    if !match_pattern(&ch.to_string(), &vec![pat[..pat.len() - 1].to_string()]) {
                        break;
                    }
                }
                *i += 1;
                iterator += 1;
            }

            return true;
        }

        pat if pat.ends_with("*") => {
            loop {
                if *i >= input.len() - 1 {
                    break;
                }
                if let Some(ch) = input.chars().nth(*i) {
                    if !match_pattern(&ch.to_string(), &vec![pat[..pat.len() - 1].to_string()]) {
                        break;
                    }
                }
                *i += 1;
            }

            return true;
        }
        _ => {
            return false;
        }
    }
}
fn match_wild_card(input: char, i: &mut usize) -> bool {
    println!("matchin wildcard {input}");
    *i += 1;
    return true;
}
