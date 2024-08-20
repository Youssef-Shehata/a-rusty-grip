use std::io;
#[allow(unused)]
pub struct Config {
    pub pattern: String,
    pub input_line: String,
}

impl Config {
    #[allow(unused)]
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
            input_line,
        })
    }
    fn extract_patterns(pattern: &str) -> Vec<String> {
        let mut pattern: Vec<String> = pattern.split('|').map(|s| s.to_string()).collect();

        for pat in pattern.iter_mut() {
            *pat = pat
                .trim()
                .chars()
                .filter(|x| *x != '(' && *x != ')')
                .collect();
        }

        pattern
    }

    fn distinguish_marks(final_pat: &mut Vec<String>, temp: &mut String) {
        if temp.is_empty() {
            return;
        }

        let mut res: Vec<String> = Vec::new();
        for letter in temp.chars() {
            match letter {
                mark @ '+' | mark @ '?' | mark @ '.' | mark @ '$' | mark @ '^' => {
                    if !res.is_empty() {
                        final_pat.push(res.join(""));
                        res.clear();
                    }
                    final_pat.push(mark.to_string());
                }
                _ => res.push(letter.to_string()),
            }
        }

        if !res.is_empty() {
            final_pat.push(res.join(""));
        }
        temp.clear();
    }

    fn tokenize_pattern(pattern: String) -> Vec<String> {
        let pattern: Vec<&str> = pattern.split("").filter(|x| !x.is_empty()).collect();
        let mut groups = String::new();
        let mut temp = String::new();
        let mut slash_flag = false;
        let mut brack_flag = false;

        let mut tokenized_pattern: Vec<String> = Vec::new();

        for (index, &i) in pattern.iter().enumerate() {
            if i == r#"\"# {
                Self::distinguish_marks(&mut tokenized_pattern, &mut temp);
                slash_flag = true;
                tokenized_pattern.push(format!("\\{}", pattern[index + 1]));
                continue;
            }
            if i == "[" {
                Self::distinguish_marks(&mut tokenized_pattern, &mut temp);
                brack_flag = true;
            }

            if !brack_flag && !slash_flag {
                temp.push_str(i);
            } else if brack_flag {
                groups.push_str(i);
            }
            slash_flag = false;

            if i == "]" {
                brack_flag = false;
                tokenized_pattern.push(groups.clone());
                groups.clear();
            }
        }

        Self::distinguish_marks(&mut tokenized_pattern, &mut temp);

        return tokenized_pattern;
    }
    fn compile_pattern(tokenized_pattern: Vec<String>) -> Vec<String> {
        let mut compiled_pattern = Vec::new();
        for pat in tokenized_pattern.into_iter() {
            match pat.as_str() {
                "?" | "+" => {
                    let target = compiled_pattern.pop().unwrap();
                    compiled_pattern.push(format!("{target}{pat}"));
                }
                _ => {
                    compiled_pattern.push(pat);
                }
            }
        }
        return compiled_pattern;
    }
    pub fn get_pattern(pattern: &str) -> Vec<Vec<String>> {
        let patterns = Self::extract_patterns(pattern);
        let mut final_pattern = Vec::new();
        for pat in patterns {
            let tokenized_pattern = Self::tokenize_pattern(pat);
            let compiled_pattern = Self::compile_pattern(tokenized_pattern);

            final_pattern.push(compiled_pattern);
        }

        dbg!(&final_pattern);
        return final_pattern;
    }
}
