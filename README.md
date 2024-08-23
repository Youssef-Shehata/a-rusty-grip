# Grep CLI Tool In Rust

## Table of Contents

1. [Introduction](#introduction)
2. [Installation](#installation)
3. [Basic Usage](#basic-usage)
4. [Soon](#Soon)

## Introduction

Trying to write the famous `grep` command in Rust , implementing almost the same functionality as the original `grep` command from scratch without using any regular expressions , and writing my very own string parsing logic.

so far you can search a string or multiple files using the following patterns and it returns wether it matches or not :

- `.` to match any character except a newline
- `[abc]` to match any character in the set abc
- `[^abc]` to match any character not in the set abc
- `\w` to match a word character (alphanumeric plus \_)
- `\d` to match a digit
- `(w$)|(^[^w])` to match either pattern
- `?` to match zero or one occurrence of the preceding character
- `+` to match one or more occurrences of the preceding character

and you can use these options to control your results :
Pattern selection and interpretation:

- `-i`, `--ignore-case` ignore case distinctions in patterns and data
- `-v`, `--invert-match` select non-matching lines
- `-h`, `--help` display this help text and exit

Output control:

- `-n`, `--line-number` print line number with output lines
- `-c`,`--count` print only a count of selected lines per FILE

## Installation

Clone this repository and you will find a small script called `grep.sh` that builds and runs the project for you .
or you can build it manually by running the following commands:

```sh
cargo build --release
```

## Basic Usage

grep [OPTION]... PATTERNS [FILE]...
Search for PATTERNS in each FILE.

```sh
    ./grep -i 'hello world' menu.h main.c
```

or

```sh
    cat main.c | ./grep -i 'hello world'
```

## Soon

- `-E` to match extended regular expressions
- `-F` to match fixed strings
- `-m` stop after a certain number of lines
- `-L` to only print names of the files with 0 matches
- `-l` to only print names of the files with any matches
- `^` to match the beginning of a line
- `$` to match the end of a line
