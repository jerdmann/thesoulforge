use std::io::{self, Read, Write};

use crate::scanner::*;
use std::fs::File;

pub struct Lox {
    had_error: bool,
}

impl Lox {
    pub fn new() -> Self {
        Lox { had_error: false }
    }

    pub fn run(&mut self, s: &str) {
        let mut sc = Scanner::new(s);
        println!("{:?}", sc.scan_tokens());
    }

    pub fn run_prompt(&mut self) {
        loop {
            print!("> ");
            io::stdout().flush();

            let mut buf = String::new();
            io::stdin().read_line(&mut buf).unwrap();
            self.run(&buf);
            self.had_error = false;
        }
    }

    pub fn run_file(&mut self, f: &str) {
        let mut buf = String::new();
        File::open(f).unwrap().read_to_string(&mut buf);
        self.run(&buf);

        if self.had_error {
            todo!();
        }
    }

    pub fn error(line: usize, msg: &str) {
        Lox::report(line, "", msg);
    }

    pub fn report(line: usize, loc: &str, msg: &str) {
        eprintln!("[line {} Error{}: {}", line, loc, msg);
    }
}