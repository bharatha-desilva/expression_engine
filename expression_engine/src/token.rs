use regex::Regex;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError {
    pub kind: ErrorKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorKind {
    Empty,
    InvalidOpenCloseParantheses,
    InvalidNumberParsed,
    InvalidExpression,
}

#[derive(Debug, Clone)]
pub enum Token<'a> {
    Number(f64),
    Variable(&'a str),
    Trigonometric(&'a str),
    Operator(&'a str),
    Parenthesis(&'a str),
}

pub fn validate(tokens: &[Token<'_>]) -> Result<(), ParseError> {
    let mut paranthesis_count = 0;
    let mut open_close = false;
    let mut all_other_count = 0;
    for token in tokens {
        if let Token::Parenthesis(p_type) = token {
            if *p_type == "(" && !open_close {
                paranthesis_count += 1;
                open_close = true;
            }
            if *p_type == ")" && open_close {
                paranthesis_count -= 1;
                open_close = false;
            }
        } else {
            all_other_count += 1
        }
    }

    if paranthesis_count != 0 {
        return Err(ParseError {
            kind: ErrorKind::InvalidOpenCloseParantheses,
        });
    }

    if all_other_count == 0 {
        return Err(ParseError {
            kind: ErrorKind::Empty,
        });
    }

    Ok(())
}

pub fn trim_parantheses<'a>(tokens: &[Token<'a>]) -> Vec<Token<'a>> {
    let mut current: &[Token<'a>] = tokens;

    while let Some(last) = current.last() {
        if let Token::Parenthesis(p_end) = last {
            if let Some(Token::Parenthesis(p_start)) = current.first() {
                println!("{}", p_start);
                println!("{}", p_end);
                let trimmed: &[Token<'_>] = &current[1..current.len() - 1];
                if *p_start == "(" && *p_end == ")" && validate(trimmed).is_ok() {
                    println!("Trimming parantheses");
                    current = trimmed;
                    continue;
                }
            }
        }
        break;
    }
    current.to_vec()
}

pub fn tokenize(expression: &str) -> Result<Vec<Token>, ParseError> {
    // Regex with named groups for categorization
    let re = Regex::new(
        r"(?P<number>\d+\.\d+|\d+)|(?P<trigonometric>sin|cosec|cos|tan|sec|cot)|(?P<variable>\w+)|(?P<operator>[+\-*/^%])|(?P<parenthesis>[()])",
    )
    .unwrap();

    let mut tokens = Vec::new();

    // Match each token and categorize it
    for cap in re.captures_iter(expression) {
        if let Some(number) = cap.name("number") {
            tokens.push(Token::Number(number.as_str().parse().map_err(|_| {
                ParseError {
                    kind: ErrorKind::InvalidNumberParsed,
                }
            })?));
        } else if let Some(operator) = cap.name("operator") {
            tokens.push(Token::Operator(operator.as_str()));
        } else if let Some(operator) = cap.name("variable") {
            tokens.push(Token::Variable(operator.as_str()));
        } else if let Some(trigonometric) = cap.name("trigonometric") {
            tokens.push(Token::Trigonometric(trigonometric.as_str()));
        } else if let Some(parenthesis) = cap.name("parenthesis") {
            tokens.push(Token::Parenthesis(parenthesis.as_str()));
        }
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_parantheses_011() {
        let tokens_before_trim = tokenize("((x))").unwrap();
        let tokens = trim_parantheses(&tokens_before_trim);

        let expected = vec!["x"];

        assert_eq!(expected.len(), tokens.len());
        for (i, token) in tokens.iter().enumerate() {
            match token {
                Token::Number(num) => assert_eq!(num, &expected[i].parse::<f64>().unwrap()),
                Token::Operator(op) => assert_eq!(*op, expected[i]),
                Token::Variable(v) => assert_eq!(*v, expected[i]),
                Token::Parenthesis(pa) => assert_eq!(*pa, expected[i]),
                Token::Trigonometric(tr) => assert_eq!(*tr, expected[i]),
            }
        }
    }

    #[test]
    fn trim_parantheses_012() {
        let tokens_before_trim = tokenize("((3x))").unwrap();
        let tokens = trim_parantheses(&tokens_before_trim);

        let expected = vec!["3", "x"];

        assert_eq!(expected.len(), tokens.len());
        for (i, token) in tokens.iter().enumerate() {
            match token {
                Token::Number(num) => assert_eq!(num, &expected[i].parse::<f64>().unwrap()),
                Token::Operator(op) => assert_eq!(*op, expected[i]),
                Token::Variable(v) => assert_eq!(*v, expected[i]),
                Token::Parenthesis(pa) => assert_eq!(*pa, expected[i]),
                Token::Trigonometric(tr) => assert_eq!(*tr, expected[i]),
            }
        }
    }

    #[test]
    fn trim_parantheses_02() {
        let tokens_before_trim = tokenize("(x*3.125)").unwrap();
        let tokens = trim_parantheses(&tokens_before_trim);

        let expected = vec!["x", "*", "3.125"];

        assert_eq!(expected.len(), tokens.len());
        for (i, token) in tokens.iter().enumerate() {
            match token {
                Token::Number(num) => assert_eq!(num, &expected[i].parse::<f64>().unwrap()),
                Token::Operator(op) => assert_eq!(*op, expected[i]),
                Token::Variable(v) => assert_eq!(*v, expected[i]),
                Token::Parenthesis(pa) => assert_eq!(*pa, expected[i]),
                Token::Trigonometric(tr) => assert_eq!(*tr, expected[i]),
            }
        }
    }

    #[test]
    fn trim_parantheses_03() {
        let tokens_before_trim = tokenize("((x*3.125))").unwrap();
        let tokens = trim_parantheses(&tokens_before_trim);

        let expected = vec!["x", "*", "3.125"];

        assert_eq!(expected.len(), tokens.len());
        for (i, token) in tokens.iter().enumerate() {
            match token {
                Token::Number(num) => assert_eq!(num, &expected[i].parse::<f64>().unwrap()),
                Token::Operator(op) => assert_eq!(*op, expected[i]),
                Token::Variable(v) => assert_eq!(*v, expected[i]),
                Token::Parenthesis(pa) => assert_eq!(*pa, expected[i]),
                Token::Trigonometric(tr) => assert_eq!(*tr, expected[i]),
            }
        }
    }

    #[test]
    fn trim_parantheses_04() {
        let tokens_before_trim = tokenize("((3x)+(x^2))").unwrap();
        let tokens = trim_parantheses(&tokens_before_trim);

        let expected = vec!["(", "3", "x", ")", "+", "(", "x", "^", "2", ")"];

        assert_eq!(expected.len(), tokens.len());
        for (i, token) in tokens.iter().enumerate() {
            match token {
                Token::Number(num) => assert_eq!(num, &expected[i].parse::<f64>().unwrap()),
                Token::Operator(op) => assert_eq!(*op, expected[i]),
                Token::Variable(v) => assert_eq!(*v, expected[i]),
                Token::Parenthesis(pa) => assert_eq!(*pa, expected[i]),
                Token::Trigonometric(tr) => assert_eq!(*tr, expected[i]),
            }
        }
    }

    #[test]
    fn test_tokenize_01() {
        let expected = vec!["x"];
        let tokens = tokenize("x").unwrap();

        assert_eq!(expected.len(), tokens.len());
        for (i, token) in tokens.iter().enumerate() {
            match token {
                Token::Variable(v) => assert_eq!(*v, expected[i]),
                _ => assert!(false),
            }
        }
    }

    #[test]
    fn test_tokenize_02() {
        let expected = vec!["3"];
        let tokens = tokenize("3").unwrap();

        assert_eq!(expected.len(), tokens.len());
        for (i, token) in tokens.iter().enumerate() {
            match token {
                Token::Number(num) => assert_eq!(num, &expected[i].parse::<f64>().unwrap()),
                _ => assert!(false),
            }
        }
    }

    #[test]
    fn test_tokenize_03() {
        let expected = vec!["3", "x"];
        let tokens = tokenize("3x").unwrap();

        assert_eq!(expected.len(), tokens.len());
        for (i, token) in tokens.iter().enumerate() {
            match token {
                Token::Number(num) => assert_eq!(num, &expected[i].parse::<f64>().unwrap()),
                Token::Variable(v) => assert_eq!(*v, expected[i]),
                _ => assert!(false),
            }
        }
    }

    #[test]
    fn test_tokenize_1() {
        let expected = vec![
            "(", "x", "*", "3", ")", "+", "3", "*", "(", "x", "*", "2", ")",
        ];
        let tokens = tokenize("(x*3) + 3 * (x*2)").unwrap();

        assert_eq!(expected.len(), tokens.len());
        for (i, token) in tokens.iter().enumerate() {
            match token {
                Token::Number(num) => assert_eq!(num, &expected[i].parse::<f64>().unwrap()),
                Token::Operator(op) => assert_eq!(*op, expected[i]),
                Token::Variable(v) => assert_eq!(*v, expected[i]),
                Token::Parenthesis(pa) => assert_eq!(*pa, expected[i]),
                Token::Trigonometric(tr) => assert_eq!(*tr, expected[i]),
            }
        }
    }

    #[test]
    fn test_tokenize_2() {
        let expected = vec![
            "(", "x", "*", "3.125", ")", "+", "3", "*", "(", "x", "*", "2.75", ")",
        ];
        let tokens = tokenize("(x*3.125) + 3 * (x*2.75)").unwrap();
        println!("expected {:?}", expected);
        println!("tokens {:?}", tokens);
        for (i, token) in tokens.iter().enumerate() {
            match token {
                Token::Number(num) => assert_eq!(num, &expected[i].parse::<f64>().unwrap()),
                Token::Operator(op) => assert_eq!(*op, expected[i]),
                Token::Variable(v) => assert_eq!(*v, expected[i]),
                Token::Parenthesis(pa) => assert_eq!(*pa, expected[i]),
                Token::Trigonometric(tr) => assert_eq!(*tr, expected[i]),
            }
        }
    }

    #[test]
    fn test_tokenize_3_0() {
        let expected = vec!["sin", "(", "x", ")"];
        let tokens = tokenize("sin(x)").unwrap();
        println!("expected {:?}", expected);
        println!("tokens {:?}", tokens);
        for (i, token) in tokens.iter().enumerate() {
            match token {
                Token::Number(num) => assert_eq!(num, &expected[i].parse::<f64>().unwrap()),
                Token::Operator(op) => assert_eq!(*op, expected[i]),
                Token::Variable(v) => assert_eq!(*v, expected[i]),
                Token::Parenthesis(pa) => assert_eq!(*pa, expected[i]),
                Token::Trigonometric(tr) => assert_eq!(*tr, expected[i]),
            }
        }
    }

    #[test]
    fn test_tokenize_3_1() {
        let expected = vec!["cos", "(", "x", ")"];
        let tokens = tokenize("cos(x)").unwrap();
        println!("expected {:?}", expected);
        println!("tokens {:?}", tokens);
        for (i, token) in tokens.iter().enumerate() {
            match token {
                Token::Number(num) => assert_eq!(num, &expected[i].parse::<f64>().unwrap()),
                Token::Operator(op) => assert_eq!(*op, expected[i]),
                Token::Variable(v) => assert_eq!(*v, expected[i]),
                Token::Parenthesis(pa) => assert_eq!(*pa, expected[i]),
                Token::Trigonometric(tr) => assert_eq!(*tr, expected[i]),
            }
        }
    }

    #[test]
    fn test_tokenize_3_2() {
        let expected = vec!["cot", "(", "x", ")"];
        let tokens = tokenize("cot(x)").unwrap();
        println!("expected {:?}", expected);
        println!("tokens {:?}", tokens);
        for (i, token) in tokens.iter().enumerate() {
            match token {
                Token::Number(num) => assert_eq!(num, &expected[i].parse::<f64>().unwrap()),
                Token::Operator(op) => assert_eq!(*op, expected[i]),
                Token::Variable(v) => assert_eq!(*v, expected[i]),
                Token::Parenthesis(pa) => assert_eq!(*pa, expected[i]),
                Token::Trigonometric(tr) => assert_eq!(*tr, expected[i]),
            }
        }
    }

    #[test]
    fn test_validate_ok() {
        let tokens = vec![
            Token::Parenthesis("("),
            Token::Variable("x"),
            Token::Parenthesis(")"),
        ];
        let tokens_ref = tokens.as_slice();
        assert_eq!(validate(tokens_ref).is_ok(), true);
    }

    #[test]
    fn test_validate_err() {
        let tokens = vec![
            Token::Parenthesis(")"),
            Token::Operator("+"),
            Token::Parenthesis("("),
        ];
        let tokens_ref = tokens.as_slice();
        assert_eq!(validate(tokens_ref).is_ok(), false);
    }
}
