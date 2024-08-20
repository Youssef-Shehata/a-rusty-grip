use crate::pattern_processor::Config;
#[allow(unused)]
pub fn grep(input: &str, pattern: &str) -> bool {
    let pattern = Config::get_pattern(pattern);
    return false;
}

fn match_pattern(input: &str, pattern: &Vec<String>) -> bool {
    let mut i = 0;
    while i < input.len() {}
    return false;
}
#[allow(unused)]
fn match_exact(input: char, pattern: &str, i: &mut usize) -> bool {
    *i += 1;
    println!("matching {} with exactly {}", input, pattern);

    return input == pattern.chars().nth(0).unwrap();
}
#[allow(unused)]
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
#[allow(unused)]
fn match_group(input: char, pattern: &str, i: &mut usize) -> bool {
    *i += 1;
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
#[allow(unused)]
fn match_symbol(input: char, pattern: &str, i: &mut usize) -> bool {
    println!("matching {} with symbol {}", input, pattern);

    *i += 1;
    match pattern {
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
    }
}
#[allow(unused)]
fn match_quantifier(input: &str, pattern: &str, i: &mut usize) -> bool {
    let target = pattern.chars().nth(0).unwrap();
    match pattern {
        pat if pat.ends_with("+") => {
            println!(
                "matching plus {} with {target}",
                input.chars().nth(*i).unwrap()
            );
            if !(input.chars().nth(*i).unwrap() == target) {
                return false;
            }
            while *i < input.len() && input.chars().nth(*i).unwrap() == target {
                *i += 1;
            }
            return true;
        }
        pat if pat.ends_with("?") => {
            println!(
                "matching ? {} with {target}",
                input.chars().nth(*i).unwrap()
            );
            loop {
                if *i == input.len() - 1 || input.chars().nth(*i).unwrap() != target {
                    break;
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
#[allow(unused)]
fn match_wild_card(input: &str, i: &mut usize) -> bool {
    if let Some(ch) = input.chars().nth(*i) {
        println!("matchin wildcard {ch}");
        *i += 1;
        return true;
    } else {
        *i += 1;
        return false;
    }
}
