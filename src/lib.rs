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
                    mark if mark == '+' || mark == '?' => {
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
    let pattern = Config::serialize_pattern(pattern);

    println!("patter : {:?}", pattern);
    return false;
}
