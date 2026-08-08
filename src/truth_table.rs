use std::{collections::HashMap, print};

fn is_valid(formula: &str) -> Result<HashMap<char, bool>, String> {
    let mut depth = 0;
    let mut m = HashMap::new();
    for char in formula.chars() {
        match char {
            '!' => continue,
            '&' | '|' | '^' | '>' | '=' => depth -= 1,
            _ => {
                if char.is_ascii_uppercase() {
                    m.insert(char, false);
                    depth += 1;
                } else {
                    return Err(format!("Invalid character: {}", char));
                }
            }
        }
        if depth <= 0 {
            return Err(format!(
                "Binary operator doesn't have possible arguments: {}",
                char
            ));
        }
    }
    if depth == 1 {
        Ok(m)
    } else {
        Err(format!(
            "'{}' ends up with multiple results, invalid RPN expression",
            formula
        ))
    }
}

fn run(formula: &mut String, map: &HashMap<char, bool>) -> Option<bool> {
    let c = formula.pop()?;
    if c.is_ascii_uppercase() {
        return Some(*map.get(&c).unwrap());
    }
    let first = run(formula, map).unwrap();
    let second = run(formula, map).unwrap_or(false);
    match c {
        '!' => Some(!first),
        '&' => Some(first & second),
        '|' => Some(first | second),
        '^' => Some(first ^ second),
        '>' => Some(!first | second),
        '=' => Some(first == second),
        _ => None,
    }
}

pub fn print_truth_table(formula: &str) {
    let mut table = is_valid(formula).unwrap();
    let mut vars: Vec<char> = table.keys().map(|c| *c).collect();
    vars.sort();
    print!("|");
    for c in &vars {
        print!(" {} |", c)
    }
    print!(" = |\n");
    print!("|");
    for _ in &vars {
        print!("---|");
    }
    print!("---|\n");
    for i in 0..(1 << vars.len()) {
        print!("|");
        for (j, c) in vars.iter().rev().enumerate().rev() {
            let val = (i >> j) & 1;
            table.insert(*c, val == 1);
            print!(" {} |", val);
        }
        let num = match run(&mut formula.to_string(), &table).unwrap() {
            true => 1,
            false => 0,
        };
        println!(" {} |", num);
    }
}
