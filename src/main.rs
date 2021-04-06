use std::io::{stdin, stdout, Write};

mod chemistry;
use chemistry::Equation;

fn main() {
    loop {
        let mut string = String::new();
        stdout().write(b"> ").unwrap();
        stdout().flush().unwrap();
        stdin().read_line(&mut string).unwrap();
        let equation = Equation::parse_str(string.as_str());
        match equation {
            Ok(eq) => {
                println!(
                    "{} - {}",
                    eq.to_string(),
                    match eq.is_balanced() {
                        true => "Balanced",
                        false => "Not Balanced",
                    },
                );
            }
            Err(e) => println!("Error: {}", e),
        }
    }
}
