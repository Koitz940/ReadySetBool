fn is_valid(formula: &str) -> Result<(), String> {
    let mut depth = 0;
    for char in formula.chars() {
        match char {
            '0' | '1' => depth += 1,
            '!' => continue,
            '&' | '|' | '^' | '>' | '=' => depth -= 1,
            _ => return Err(format!("Invalid character: {}", char)),
        }
        if depth <= 0 {
            return Err(format!("Binary operator doesn't have possible arguments: {}", char));
        }
    }
    if depth == 1 {
		Ok(())
	} else {
		Err(format!("'{}' ends up with multiple results, invalid RPN expression", formula))
	}
}

fn run(formula: &mut String) -> Option<bool> {
    let c = formula.pop()?;
    if c == '0' || c == '1' {
        return Some(c == '1');
    }
    let first = run(formula).unwrap();
    let second = run(formula).unwrap_or(false);
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

//Idiomatic Rust function would not use assert, it would just return a Result<bool, &str>, I just want to follow the subject
pub fn eval_formula(formula: &str) -> bool {
    is_valid(formula).unwrap();
    let mut formula = formula.to_string();

    run(&mut formula).unwrap()
}

fn test1eval_formula(formula: &str, expected: bool) {
    assert!(
        expected == eval_formula(formula),
        "'{}' did not evaluate to the right boolean, expected {}, got {}",
        formula,
        expected,
        !expected
    );
}

pub fn test_eval_formula() {
    test1eval_formula("10&", false);
    test1eval_formula("10|", true);
    test1eval_formula("10>", true);
    test1eval_formula("10=", false);
    test1eval_formula("1011||=", true);
    test1eval_formula("1011||=!", false);
}
