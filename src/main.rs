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
fn grep(input: &str, pattern: &str) -> bool {
    let pattern: Vec<&str> = pattern.split("").filter(|x| *x != "").collect();
    let mut groups = String::new();
    let mut slash_flag = false;
    let mut brack_flag = false;
    let mut final_pat: Vec<String> = Vec::new();

    for (index, i) in pattern.iter().enumerate() {
        if i.to_string() == r#"\"# {
            slash_flag = true;
            final_pat.push(format!("\\{}", pattern[index + 1]));
            continue;
        }
        if i.to_string() == "[" {
            brack_flag = true;
        }
        if !brack_flag && !slash_flag {
            final_pat.push(i.to_string());
        } else if !slash_flag {
            groups = groups + i;
        }
        slash_flag = false;

        if i.to_string() == "]" {
            brack_flag = false;
            final_pat.push(groups);
            groups = String::from("");
        }
    }
    if match_wild_patterns(String::from(input), &final_pat) {
        println!("true");
        return true;
        //process::exit(0)
    }

    println!("false");
    return false;
    //process::exit(1)
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
    grep(&input_line, &pattern);
}
fn match_wild_patterns(inputline: String, pattern: &Vec<String>) -> bool {
    for (index, letter) in inputline.chars().enumerate() {
        if &pattern[0] == "^" {
            for (x, i) in pattern[1..].iter().enumerate() {
                println!("{i}");
                if let Some(input) = inputline.chars().nth(x) {
                    if !match_pattern(&input.to_string(), i) {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            return true;
        }
        if match_pattern(&letter.to_string(), &pattern[0]) {
            for (x, i) in pattern.iter().enumerate() {
                if let Some(input) = inputline.chars().nth(x + index) {
                    if !match_pattern(&input.to_string(), i) {
                        println!("{input} : {i}");
                        break;
                    } else if x == pattern.len() - 1 {
                        println!("finale : {input} : {i}");
                        return true;
                    }
                } else {
                    return false;
                }
            }
        }
    }
    return false;
}

#[cfg(test)]
mod true_tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(grep("ass", "ass"));
    }

    #[test]
    fn case2() {
        assert!(grep("2", "\\d"));
    }
    #[test]
    fn case3() {
        assert!(grep("012", "\\d\\d\\d[sa]"));
    }
    #[test]
    fn case4() {
        assert!(grep("oopspp", "[so]"));
    }
    #[test]
    fn case5() {
        assert!(grep("019248apapopopiw23", "[^nmbv]"));
    }
    #[test]
    fn case6() {
        assert!(grep("qwe", "[sw]"));
    }
    #[test]
    fn case7() {
        assert!(grep("d2d apple", "\\w\\d\\w apple"));
    }
    #[test]
    fn case8() {
        assert!(grep("22w a", "\\d\\dw [sa]"));
    }
    #[test]
    fn case9() {
        assert_eq!(grep("opac", "[^c]"), true);
    }
}

#[cfg(test)]
mod false_tests {
    use super::*;

    #[test]
    fn case1() {
        assert_ne!(grep("w", "\\d"), true);
    }

    #[test]
    fn case2() {}
    #[test]
    fn case3() {
        assert_ne!(grep("w29d", "[sa]"), true);
    }
    #[test]
    fn case4() {
        assert_ne!(grep("dsx", "d[pw]x"), true);
    }
    #[test]
    fn case5() {
        assert_ne!(grep("12 ds 21", "12 ds [^2]1"), true);
    }
    #[test]
    fn case6() {
        assert_ne!(grep("daas", "^aas"), true);
    }
    #[test]
    fn case7() {
        assert_ne!(grep("da", "^das"), true);
    }
    #[test]
    fn case8() {
        assert_ne!(grep("ad", "^d"), true);
    }
    #[test]
    fn case9() {
        assert_ne!(grep("1p", "^1 "), true);
    }
    #[test]
    fn case10() {
        assert_ne!(grep("22w ", "\\d\\dw [^sa]"), true);
    }
}
