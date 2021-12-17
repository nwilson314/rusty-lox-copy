use std::fs;
use std::io::Write;
use std::process;

pub struct Lox {
    had_error: bool
}

impl Lox {
    pub fn new() -> Self {
        Lox { had_error: false }
    }

    pub fn run_file(&mut self, path: &String) {
        let contents = fs::read_to_string(path)
            .expect("Something went wrong reading the script");
        self.run(&contents);

        if self.had_error {
            process::exit(65);
        }
    }

    pub fn run_prompt(&mut self) {
        let mut line = String::new();
    
        loop {
            print!("> ");
            match std::io::stdout().flush() {
                Ok(_) => (),
                Err(error) => eprintln!("{}", error)
            }
            match std::io::stdin().read_line(&mut line) {
                Ok(_) => (),
                Err(error) => eprintln!("{}", error)
            }
            self.run(&line);
            self.had_error = false;
            line.clear();
        }
    }

    fn run(&self, source: &String) {
        // scanner: Scanner = Scanner::new(source);
        // tokens: Vec<Token> = scanner.scan_tokens();
    
        // for token in tokens {
        //     println!("{}", token);
        // }
    
        println!("{}", source);
    }

    pub fn error(&mut self, line: u32, message: &String) {
        self.report(line, &String::from(""), message);
    }

    fn report(&mut self, line: u32, loc: &String, message: &String) {
        eprintln!("[line {}] Error {}: {}", line, loc, message);
        self.had_error = true;
    }
}