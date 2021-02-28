use regex::Regex;
use std::{cmp::Ordering::{Greater, Less}, io::{Write, stdin, stdout}};

fn main() {
    loop {
        let mut string = String::new();
        stdout().write(b"> ").unwrap();
        stdout().flush().unwrap();
        stdin().read_line(&mut string).unwrap();
        let equation = ChemEquation::parse_str(string.as_str());
        match equation {
            Ok(eq) => println!("{:?}", equation),
            Err(e) => println!("Error: {}", e)
        }
    }
}
#[derive(Clone, Debug)]
struct ChemMolecule {
    name: String,
    multiple: u32,
}
#[derive(Clone, Debug)]
struct ChemToken {
    multiple: u32,
    term: Vec<ChemMolecule>,
}

#[derive(Clone, Debug)]
struct ChemEquation {
    lhs: Vec<ChemToken>,
    rhs: Vec<ChemToken>,
}

impl ChemEquation {
    fn parse_side(s: &str) -> Result<Vec<ChemToken>, String> {
        let mut tokens: Vec<_> = vec![];
        for token_str in s.split("+").map(|x| x.trim()) {
            let mut token = ChemToken {
                multiple: 0,
                term: vec![],
            };
            let mut molecule = ChemMolecule {
                multiple: 0,
                name: String::new()
            };
            for (i, char) in token_str.chars().enumerate() {
                if char.is_numeric() {
                    if i == 0 {
                        token.multiple = token.multiple * 10 + char.to_digit(10).unwrap();
                    } else {
                        molecule.multiple = molecule.multiple * 10 + char.to_digit(10).unwrap();
                    }
                } else if char.is_ascii_alphabetic() {
                    if char.is_uppercase() {
                        molecule.name = char.to_string();
                    } else {
                        if molecule.multiple == 0 {
                            molecule.multiple = 1;
                        }
                        molecule.name.push(char);
                        token.term.push(molecule.clone());
                    }
                } else {
                    return Err(format!("Unexpected token \"{}\" at {}", char, i + 1));
                }
            }
            if token.multiple == 0 { token.multiple = 1; }
            tokens.push(token);
        }
        Ok(tokens)
    }
    fn parse_str(s: &str) -> Result<Self, String> {
        let re = Regex::new("=|->|-->").unwrap();
        let sides: Vec<_> = re.split(s).collect();
        match sides.len().cmp(&2) {
            Greater => Err(String::from("Too many sides")),
            Less => Err(String::from("Too few sides")),
            _ => {
                let lhs = ChemEquation::parse_side(sides[0])?;
                let rhs = ChemEquation::parse_side(sides[1])?;
                Ok(ChemEquation { lhs, rhs })
            }
        }
    }
}
