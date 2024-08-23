use std::io::{self, Read};
use std::process::{self, exit};
use std::{fs, string};

const HELP_MESSAGE: &str = r#"
Usage: grep [OPTION]... PATTERNS [FILE]...
Search for PATTERNS in each FILE.
Example: grep -i 'hello world' menu.h main.c
PATTERNS can contain multiple patterns separated by newlines.

Pattern selection and interpretation:
  -E, --extended-regexp     PATTERNS are extended regular expressions
  -F, --fixed-strings       PATTERNS are strings
  -i, --ignore-case         ignore case distinctions in patterns and data

  -v, --invert-match        select non-matching lines
  -h, --help                display this help text and exit

Output control:
  -m, --max-count=NUM       stop after NUM selected lines
  -n, --line-number         print line number with output lines
  -L, --files-without-match print only names of FILEs with no selected lines
  -l, --files-with-matches  print only names of FILEs with selected lines
  -c, --count               print only a count of selected lines per FILE
"#;

#[allow(unused)]
pub struct File {
    pub name: String,
    pub buffer: Vec<String>,
}
impl File {
    pub fn new(name: String, buffer: Vec<String>) -> File {
        File { name, buffer }
    }
}
#[allow(unused)]
pub struct Flags {
    pub case_insenstive: bool,
    pub invert_match: bool,
    pub count: bool,
    pub line_numbers: bool,
    pub ere: bool,
    pub fixed_string: bool,
    pub marks: Vec<char>,
}
impl Flags {
    pub fn new() -> Flags {
        Self {
            case_insenstive: false,
            invert_match: false,
            count: false,
            line_numbers: false,
            ere: false,
            fixed_string: false,
            marks: vec!['+', '?', '.', '$', '^', '*'],
        }
    }
}
#[allow(unused)]
pub enum InputEnum {
    FileInput(Vec<File>),
    StdInput(Vec<String>),
}
#[allow(unused)]
pub struct Config {
    pub pattern: Vec<Vec<String>>,
    pub input_lines: InputEnum,
    pub flags: Flags,
}
impl Config {
    #[allow(unused)]
    pub fn new(input: &[String]) -> Result<Config, String> {
        if input.len() < 2 {
            return Err("Not enough arguments".to_string());
        }

        let mut flags = Flags::new();
        let mut args = Vec::new();
        for arg in input[1..].iter() {
            match arg.as_str() {
                s if s.starts_with("-") => {
                    let s_vec: Vec<&str> = s[1..].split("").filter(|x| *x != "").collect();
                    for i in s_vec.iter() {
                        match *i {
                            "i" | "-ignore-case" => {
                                flags.case_insenstive = true;
                            }
                            "v" | "-invert-match" => {
                                flags.invert_match = true;
                            }
                            "c" | "-count" => {
                                flags.count = true;
                            }
                            "n" | "-line-number" => {
                                flags.line_numbers = true;
                            }
                            "E" | "-extended-regexp" => {
                                flags.ere = true;
                            }
                            "F" | "-fixed-strings" => flags.fixed_string = true,

                            "h" | "-help" => {
                                println!(r#"{HELP_MESSAGE}"#);
                                process::exit(0);
                            }

                            _ => {
                                return Err(format!(
                                    "Uknown option -{}, Use -h to list all available options.",
                                    *i
                                ));
                            }
                        }
                    }
                }
                _ => {
                    args.push(arg.clone());
                }
            }
        }

        if let None = args.get(0) {
            return Err("No pattern Provided".to_string());
        }
        let pattern = args[0].clone();
        let pattern = Self::pattern_parser(&pattern);
        let input_lines = Self::read_input(args)?;
        Ok(Config {
            pattern,
            input_lines,
            flags,
        })
    }
    fn read_input(args: Vec<String>) -> Result<InputEnum, String> {
        let mut files = Vec::new();
        if !args[1..].is_empty() {
            for arg in args[1..].iter() {
                let mut file_content = String::new();
                match fs::read_to_string(arg) {
                    Ok(contents) => {
                        file_content.push_str(&contents);
                    }
                    Err(e) => {
                        return Err("reading file : {e}".to_string());
                    }
                }

                let line: Vec<String> = file_content
                    .trim_end()
                    .split("\n")
                    .map(|x| x.to_string())
                    .collect();
                let file = File::new(arg.clone(), line);
                files.push(file);
            }
            if files.len() > 1 {
                return Ok(InputEnum::FileInput(files));
            } else {
                return Ok(InputEnum::StdInput(files[0].buffer.clone()));
            }
        }

        let mut std_input = String::new();
        io::stdin().read_to_string(&mut std_input).unwrap();

        let line: Vec<String> = std_input
            .trim_end()
            .split("\n")
            .map(|x| x.to_string())
            .collect();
        Ok(InputEnum::StdInput(line))
    }

    pub fn pattern_parser(pattern: &str) -> Vec<Vec<String>> {
        let mut patterns: Vec<String> = pattern.split('|').map(|s| s.to_string()).collect();
        for pat in patterns.iter_mut() {
            *pat = pat
                .trim()
                .chars()
                .filter(|x| *x != '(' && *x != ')')
                .collect();
        }
        let mut final_pattern = Vec::new();
        for pat in patterns {
            let tokenized_pattern = Self::tokenize_pattern(pat);
            let compiled_pattern = Self::compile_pattern(tokenized_pattern);

            final_pattern.push(compiled_pattern);
        }
        final_pattern
    }

    fn distinguish_marks(final_pat: &mut Vec<String>, temp: &mut String) {
        if temp.is_empty() {
            return;
        }

        let mut res: Vec<String> = Vec::new();
        for letter in temp.chars() {
            match letter {
                mark @ '+' | mark @ '?' | mark @ '.' | mark @ '$' | mark @ '^' | mark @ '*' => {
                    if !res.is_empty() {
                        final_pat.extend(res.clone());
                        res.clear();
                    }
                    final_pat.push(mark.to_string());
                }
                _ => res.push(letter.to_string()),
            }
        }

        if !res.is_empty() {
            final_pat.extend(res.clone());
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
                "?" | "+" | "*" => {
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
}
