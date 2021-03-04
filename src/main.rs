use regex::Regex;
use std::{
    cmp::Ordering::{Greater, Less},
    collections::HashMap,
    io::{stdin, stdout, Write},
};

fn main() {
    loop {
        let mut string = String::new();
        stdout().write(b"> ").unwrap();
        stdout().flush().unwrap();
        stdin().read_line(&mut string).unwrap();
        let equation = ChemEquation::parse_str(string.as_str());
        match equation {
            Ok(eq) => {
                println!(
                    "{}",
                    match eq.is_balanced() {
                        true => "balanced",
                        false => "not balanced",
                    }
                );
            }
            Err(e) => println!("Error: {}", e),
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
    terms: Vec<ChemMolecule>,
}

#[derive(Clone, Debug)]
struct ChemEquation {
    lhs: Vec<ChemToken>,
    rhs: Vec<ChemToken>,
}

impl ChemEquation {
    fn parse_side(s: &str) -> Result<Vec<ChemToken>, String> {
        let re = Regex::new(r"(?:(\d*)([A-Z]{1}[a-z]{0,2})(\d*))").unwrap();
        let tokens = s
            .split("+")
            .map(|token_str| {
                let token_str = token_str.trim();
                let mut term_multiple = 1u32;
                let molecules = re
                    .captures_iter(token_str)
                    .map(|x| {
                        if let Some(mul) = x.get(1) {
                            if let Ok(val) = mul.as_str().parse() {
                                term_multiple = val;
                            }
                        }
                        let name = String::from(&x[2]);
                        let multiple = if let Some(mul) = x.get(3) {
                            mul.as_str().parse().unwrap_or(1u32)
                        } else {
                            1u32
                        };
                        ChemMolecule { name, multiple }
                    })
                    .collect();
                ChemToken {
                    terms: molecules,
                    multiple: term_multiple,
                }
            })
            .collect();
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
    fn yes(side: &Vec<ChemToken>) -> HashMap<&String, u32> {
        let mut map = HashMap::new();
        for i in side.iter() {
            for j in i.terms.iter() {
                let count = map.entry(&j.name).or_insert(0);
                *count += i.multiple * j.multiple;
            }
        }
        map
    }
    fn is_balanced(self) -> bool {
        let lhs_map = ChemEquation::yes(&self.lhs);
        let rhs_map = ChemEquation::yes(&self.rhs);
        lhs_map == rhs_map
    }
}
