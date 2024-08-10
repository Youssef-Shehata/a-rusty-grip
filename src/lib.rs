use std::io;

pub struct Config {
    pub pattern: String,
    pub input_line: String,
}
#[derive(Debug)]
enum WhatWematchin {
    Exact(String),
    EndOfLine(String),
    BeginningOfLine(String),
    Group(String),
    Symbol(String),
    Quantifier(String),
}
impl Config {
    pub fn new(input: &[String]) -> Result<Config, &'static str> {
        if input.len() < 3 {
            return Err("not enough arguments");
        }
        let pattern = input[2].clone();

        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        input_line = input_line.trim_end_matches('\n').to_string();

        Ok(Config {
            pattern,
            input_line: input_line.to_string(),
        })
    }
    fn empty_temp(final_pat: &mut Vec<String>, temp: &mut String) {
        if !temp.is_empty() {
            let mut res: Vec<String> = Vec::new();
            for letter in temp.chars() {
                match letter {
                    mark if mark == '+' => {
                        let quantifier = res.pop().unwrap_or(String::from(""));
                        if !res.is_empty() {
                            final_pat.push(res.join(""));
                            res.clear();
                        }
                        final_pat.push(quantifier + &mark.to_string());
                        continue;
                    }
                    _ => (),
                }
                res.push(String::from(letter));
            }

            if !res.is_empty() {
                final_pat.push(res.join(""));
            }
            temp.clear();
        }
    }
    fn serialize_pattern(pattern: &str) -> Vec<WhatWematchin> {
        let pattern: Vec<&str> = pattern.split("").filter(|x| *x != "").collect();
        let mut groups = String::new();
        let mut temp: String = String::new();
        let mut slash_flag = false;
        let mut brack_flag = false;
        let mut final_pat: Vec<String> = Vec::new();

        for (index, i) in pattern.iter().enumerate() {
            if i.to_string() == r#"\"# {
                Self::empty_temp(&mut final_pat, &mut temp);
                slash_flag = true;
                final_pat.push(format!("\\{}", pattern[index + 1]));
                continue;
            }
            if i.to_string() == "[" {
                Self::empty_temp(&mut final_pat, &mut temp);
                brack_flag = true;
            }
            if !brack_flag && !slash_flag {
                temp = temp + i;
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

        Self::empty_temp(&mut final_pat, &mut temp);
        dbg!(&final_pat);
        let mut pattern_enum = Vec::new();
        for pat in &final_pat {
            if pat.starts_with("[") {
                pattern_enum.push(WhatWematchin::Group(String::from(pat)));
            } else if pat.starts_with("\\") {
                pattern_enum.push(WhatWematchin::Symbol(String::from(pat)));
            } else if pat.starts_with("^") {
                pattern_enum.push(WhatWematchin::BeginningOfLine(String::from(pat)));
            } else if pat.ends_with("$") {
                pattern_enum.push(WhatWematchin::EndOfLine(String::from(pat)));
            } else if pat.starts_with("[") {
                pattern_enum.push(WhatWematchin::Group(String::from(pat)));
            } else if pat.ends_with("+") {
                pattern_enum.push(WhatWematchin::Quantifier(String::from(pat)));
            } else {
                pat.split("").filter(|&x| x != "").for_each(|x| {
                    pattern_enum.push(WhatWematchin::Exact(String::from(x)));
                });
            }
        }
        dbg!(&pattern_enum);
        return pattern_enum;
    }
}
pub fn grep(input: &str, pattern: &str) -> bool {
    println!("Matching {} With {}", input, pattern);
    let mut i = 0;
    let pattern = Config::serialize_pattern(pattern);
    while i < input.len() {
        for (index, pat) in pattern.iter().enumerate() {
            let global_flag: bool = match pat {
                WhatWematchin::Exact(pattern) => {
                    match_exact(input.chars().nth(i).unwrap(), &pattern, &mut i)
                }

                WhatWematchin::EndOfLine(pattern) => line_matches(&input, &pattern, true, &mut i),
                WhatWematchin::BeginningOfLine(pattern) => {
                    line_matches(&input, &pattern, false, &mut i)
                }
                WhatWematchin::Group(pattern) => {
                    match_group(input.chars().nth(i).unwrap(), &pattern, &mut i)
                }

                WhatWematchin::Symbol(pattern) => {
                    match_symbol(input.chars().nth(i).unwrap(), &pattern, &mut i)
                }
                WhatWematchin::Quantifier(pattern) => match_quantifier(&input, &pattern, &mut i),
            };

            if global_flag && index == pattern.len() - 1 {
                return true;
            } else if !global_flag || i == input.len() {
                break;
            }
        }
    }
    return false;
}
fn match_exact(input: char, pattern: &str, i: &mut usize) -> bool {
    *i += 1;
    println!("matching {} with exactly {}", input, pattern);

    return input == pattern.chars().nth(0).unwrap();
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
        _ => {
            return false;
        }
    }
}
//--------------------------------------------------------------//
//TESTS
//______________________________________________________________//

#[cfg(test)]
pub mod exact {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(grep("wd", "d"), true);
    }

    #[test]
    fn case2() {
        assert!(grep("ass", "ass"));
    }
}
#[cfg(test)]

pub mod symbols {
    use super::*;
    #[test]
    fn case3() {
        assert!(grep("2", "\\d"));
    }
    #[test]
    fn case4() {
        assert!(grep("012", "\\d\\d\\d"));
    }
}
#[cfg(test)]
pub mod groups {
    use super::*;

    #[test]
    fn case5() {
        assert_ne!(grep("w29d", "[sa]"), true);
    }
    #[test]
    fn case6() {
        assert!(grep("oopspp", "[so]"));
    }
    #[test]
    fn case7() {
        assert!(grep("019248apapopopiw23", "[^nmbv]"));
    }
    #[test]
    fn case8() {
        assert!(grep("qwe", "[sw]"));
    }
}
#[cfg(test)]
pub mod combinations {

    use super::*;
    #[test]
    fn case9() {
        assert!(grep("d2d apple", "\\w\\d\\w apple"));
    }
    #[test]
    fn case10() {
        assert!(grep("22w a", "\\d\\dw [sa]"));
    }
    #[test]
    fn case11() {
        assert_ne!(grep("opac", "[^c]"), true);
    }
    #[test]
    fn case15() {
        assert_ne!(grep("dsx", "d[pw]x"), true);
    }
    #[test]
    fn case16() {
        assert_ne!(grep("12 ds 21", "12 ds [^2]1"), true);
    }
    #[test]
    fn case17() {
        assert_ne!(grep("22w ", "\\d\\dw [^sa]"), true);
    }
}
#[cfg(test)]
pub mod beginning_of_line {
    use super::*;
    #[test]
    fn case12() {
        assert_eq!(grep("opac", "^opa"), true);
    }
    #[test]
    fn case13() {
        assert_eq!(grep("opac", "^o"), true);
    }
    #[test]
    fn case14() {
        assert_eq!(grep("a", "^a"), true);
    }
    #[test]
    fn case18() {
        assert_ne!(grep("da", "^das"), true);
    }
    #[test]
    fn case19() {
        assert_ne!(grep("ad", "^d"), true);
    }
    #[test]
    fn case20() {
        assert_ne!(grep("1p", "^1 "), true);
    }
    #[test]
    fn case21() {
        assert_ne!(grep("daas", "^aas"), true);
    }
    #[test]
    fn case22() {
        assert_ne!(grep("slog", "^log"), true);
    }
}

#[cfg(test)]
pub mod end_of_line {
    use super::*;

    #[test]
    fn case23() {
        assert_ne!(grep("man ", "man$"), true);
    }
    #[test]
    fn case24() {
        assert!(grep("o", "o$"));
    }
    #[test]
    fn case25() {
        assert!(grep("mad man", "man$"));
    }
    #[test]
    fn case26() {
        assert!(grep("qwe  ", "  $"));
    }
}
#[cfg(test)]
pub mod plus {
    use super::*;

    #[test]
    fn case27() {
        assert_eq!(grep("man ", "ma+n"), true);
    }
    #[test]
    fn case28() {
        assert_eq!(grep("maan ", "ma+n"), true);
    }
    #[test]
    fn case29() {
        assert_ne!(grep("mn ", "ma+n"), true);
    }
    #[test]
    fn case30() {
        assert_eq!(grep("aan ", "a+n"), true);
    }
    #[test]
    fn case31() {
        assert_eq!(grep("maa ", "ma+"), true);
    }
}
