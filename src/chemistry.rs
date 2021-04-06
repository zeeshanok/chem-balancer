pub use chemistry::{Equation, Molecule, Token};

pub mod chemistry {
    use regex::Regex;
    use std::{
        cmp::Ordering::{Greater, Less},
        collections::HashMap,
    };

    #[derive(Clone, Debug)]
    pub struct Molecule {
        name: String,
        multiple: u32,
    }
    #[derive(Clone, Debug)]
    pub struct Token {
        multiple: u32,
        terms: Vec<Molecule>,
    }

    #[derive(Clone, Debug)]
    pub struct Equation {
        lhs: Vec<Token>,
        rhs: Vec<Token>,
    }

    impl Molecule {
        pub fn to_string(&self) -> String {
            match self.multiple {
                1 => self.name.clone(),
                other => format!(
                    "{}{}",
                    self.name,
                    other
                        .to_string()
                        .chars()
                        .map(|c| subscript(c).unwrap().to_string())
                        .collect::<Vec<_>>()
                        .join("")
                ),
            }
        }

        pub fn from_string(string: &String) -> Option<Self> {
            let re = Regex::new(r"([A-Za-z]+)(\d*)").unwrap();
            let captures = re.captures(string.as_str())?;
            let name = captures.get(1)?.as_str().to_string();
            let multiple = captures.get(2).unwrap().as_str();
            let multiple = if multiple.is_empty() {
                1
            } else {
                multiple.parse().unwrap()
            };
            Some(Molecule { name, multiple })
        }
    }

    impl Token {
        pub fn to_string(&self) -> String {
            let token = self
                .terms
                .iter()
                .map(|term| term.to_string())
                .collect::<Vec<_>>()
                .join("");
            match self.multiple {
                1 => token,
                other => format!("{}{}", other, token),
            }
        }
        // pub fn from_string(token_str: String) -> Self {
        //     let re = Regex::new(r"(?:(\d*)([A-Z]{1}[a-z]{0,2})(\d*))").unwrap();
        //     let token_str = token_str.trim();
        //     let mut term_multiple = 1u32;
        //     let molecules = re
        //         .captures_iter(token_str)
        //         .map(|x| {
        //             if let Some(mul) = x.get(1) {
        //                 if let Ok(val) = mul.as_str().parse() {
        //                     term_multiple = val;
        //                 }
        //             }
        //             let name = String::from(&x[2]);
        //             let multiple = if let Some(mul) = x.get(3) {
        //                 mul.as_str().parse().unwrap_or(1u32)
        //             } else {
        //                 1u32
        //             };
        //             Molecule { name, multiple }
        //         })
        //         .collect();
        //     Token {
        //         terms: molecules,
        //         multiple: term_multiple,
        //     }
        // }
        pub fn from_string(token_str: String) -> Self {
            let token_str = token_str.trim();
            let mut splits = split_caps(token_str.trim().to_string());
            let multiple = match splits[0].parse() {
                Ok(val) => {
                    splits.remove(0);
                    val
                },
                _ => 1u32,
            };
            Token {
                multiple,
                terms: splits
                    .iter()
                    .map(Molecule::from_string)
                    .map(|x| x.unwrap())
                    .collect(),
            }
        }
    }

    impl Equation {
        pub fn parse_side(s: &str) -> Result<Vec<Token>, String> {
            let tokens = s
                .split("+")
                .map(|token_str| Token::from_string(token_str.to_string()))
                .collect();
            Ok(tokens)
        }
        pub fn parse_str(s: &str) -> Result<Self, String> {
            let re = Regex::new("=|->|-->").unwrap();
            let sides: Vec<_> = re.split(s).collect();
            match sides.len().cmp(&2) {
                Greater => Err(String::from("Too many sides")),
                Less => Err(String::from("Too few sides")),
                _ => {
                    let lhs = Equation::parse_side(sides[0])?;
                    let rhs = Equation::parse_side(sides[1])?;
                    Ok(Equation { lhs, rhs })
                }
            }
        }
        pub fn side_hash(side: &Vec<Token>) -> HashMap<&String, u32> {
            let mut map = HashMap::new();
            for i in side.iter() {
                for j in i.terms.iter() {
                    let count = map.entry(&j.name).or_insert(0);
                    *count += i.multiple * j.multiple;
                }
            }
            map
        }
        pub fn is_balanced(&self) -> bool {
            let lhs_map = Equation::side_hash(&self.lhs);
            let rhs_map = Equation::side_hash(&self.rhs);
            lhs_map == rhs_map
        }
        pub fn to_string(&self) -> String {
            let func = |v: &Vec<Token>| {
                v.iter()
                    .map(|token| token.to_string())
                    .collect::<Vec<_>>()
                    .join(" + ")
            };
            format!("{} -> {}", (func)(&self.lhs), (func)(&self.rhs).as_str())
        }
    }

    pub fn subscript(num: char) -> Option<char> {
        Some(match num {
            '0' => '₀',
            '1' => '₁',
            '2' => '₂',
            '3' => '₃',
            '4' => '₄',
            '5' => '₅',
            '6' => '₆',
            '7' => '₇',
            '8' => '₈',
            '9' => '₉',
            _ => return None,
        })
    }

    pub fn split_caps(string: String) -> Vec<String> {
        let mut last = String::new();
        let mut splits = vec![];
        for chr in string.chars() {
            if !last.is_empty() {
                if chr.is_ascii_uppercase() {
                    splits.push(last.clone());
                    last.clear();
                }
            }
            last.push(chr);
        }
        splits.push(last);
        splits
    }
}

pub trait IsNumber {
    fn is_num(&self) -> bool;
}

impl IsNumber for String {
    fn is_num(&self) -> bool {
        self.chars().all(|x| x.is_digit(10))
    }
}

#[allow(dead_code, unused_variables)]
pub fn cool_fn() {
    let string = String::new();
    let is_num = string.is_num(); // make your own methods for builtin structs
}