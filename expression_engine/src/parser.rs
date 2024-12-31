use crate::node::*;
use crate::operator::*;
use crate::token::*;
use crate::trigonometric::TrigonometricFunction;

///
/// Parse text expression into expression object
///
/// # Examples
///
/// ```
/// use expression_engine::{parse, ExpressionNode, ExpressionNodeType, Operator};
/// use std::f32::consts::PI;
///
/// const EPSILON: f64 = 0.000000001;
///
/// let test = |exp_str: &str, input: f64| -> f64 {
///     parse(exp_str).unwrap().evaluate("x", input).unwrap()
/// };
///
/// assert_eq!(test("x", 1.5), 1.5);
/// assert_eq!(test("x^2", 1.5), 2.25);
/// assert_eq!(test("x^2 + x^3", 1.5), 5.625);
/// assert!((test("sin x", PI as f64 / 2.0) - 1.0).abs() < EPSILON);
///
///  ```
///
pub fn parse(expression: &str) -> Result<ExpressionNode, ParseError> {
    let tokens = tokenize(expression)?;
    let tokens_ref = tokens.as_slice();
    validate(tokens_ref)?;
    let root_node = parse_token(tokens_ref)?;
    Ok(root_node)
}

fn find_operator(current: &[Token<'_>]) -> Option<(usize, Operator)> {
    let mut min_i: usize = 0;
    let mut min_op_i: Option<Operator> = None;
    let mut paranthesis_count = 0;
    for (i, token) in current.iter().enumerate() {
        match token {
            Token::Operator(op_str) => match min_op_i {
                Some(mop) => {
                    if paranthesis_count == 0 {
                        let cop = Operator::from(&op_str).unwrap();
                        if mop > cop {
                            min_op_i = Some(cop);
                            min_i = i;
                        }
                    }
                }
                None => {
                    if paranthesis_count == 0 {
                        min_op_i = Operator::from(&op_str);
                        min_i = i
                    }
                }
            },
            Token::Parenthesis(p_type) => {
                if *p_type == "(" {
                    paranthesis_count += 1;
                }
                if *p_type == ")" {
                    paranthesis_count -= 1;
                }
            }
            _ => (),
        }
    }

    if let Some(op) = min_op_i {
        return Some((min_i, op));
    }

    None
}

fn parse_token(tokens: &[Token<'_>]) -> Result<ExpressionNode, ParseError> {
    let current = &trim_parantheses(tokens);

    let op_red = find_operator(&current);

    if let Some((min_i, op)) = op_red {
        println!("Lowerest Op {:?}, Index : {:?}", op, min_i);
        let left = trim_parantheses(&current[0..min_i]);
        let right = trim_parantheses(&current[(min_i + 1)..current.len()]);

        println!("Left {:?}", left);
        println!("Right {:?}", right);

        let left_operand;
        if left.len() == 1 {
            left_operand = map_operand(&left[0])?;
        } else {
            left_operand = ExpressionNodeType::NodeOperand(Box::new(parse_token(&left)?))
        }

        let right_operand;
        if right.len() == 1 {
            right_operand = map_operand(&right[0])?;
        } else {
            right_operand = ExpressionNodeType::NodeOperand(Box::new(parse_token(&right)?))
        }

        let exp = ExpressionNode::new(left_operand, Some(op), Some(right_operand));

        return Ok(exp);
    }

    if current.len() > 1 {
        match current[0] {
            //Token::Number(_) => map_operand(&current[0]),
            //Token::Variable(_) => &current[0..1],
            Token::Trigonometric(trig) => {
                let trig_fn = TrigonometricFunction::from(trig);
                let trig_node = match trig_fn {
                    Some(trig) => Some(ExpressionNode::new(
                        ExpressionNodeType::TrigonometricOperand((
                            trig,
                            Box::new(parse_token(&current[1..current.len()])?),
                        )),
                        None,
                        None,
                    )),
                    None => None,
                };

                match trig_node {
                    Some(node) => return Ok(node),
                    None => (),
                }
            }
            _ => (),
        }
    }

    if current.len() == 1 {
        return Ok(ExpressionNode::new(map_operand(&current[0])?, None, None));
    }

    Err(ParseError {
        kind: ErrorKind::Empty,
    })
}

fn map_operand(operand: &Token<'_>) -> Result<ExpressionNodeType, ParseError> {
    Ok(match operand {
        Token::Number(val) => ExpressionNodeType::ValueOperand(*val),
        Token::Variable(var) => ExpressionNodeType::VariableOperand(var.chars().next().unwrap()),
        Token::Trigonometric(var) => {
            ExpressionNodeType::VariableOperand(var.chars().next().unwrap())
        }
        _ => ExpressionNodeType::VariableOperand('?'),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_operator_01() {
        let tokens = tokenize("x+1").unwrap();
        let res = find_operator(&tokens);
        assert_eq!(res.is_some(), true);
        assert_eq!(res.unwrap().0, 1);
        assert_eq!(res.unwrap().1, Operator::Add);
    }

    #[test]
    fn find_operator_02() {
        let tokens = tokenize("x+1+0.5").unwrap();
        let res = find_operator(&tokens);
        assert_eq!(res.is_some(), true);
        assert_eq!(res.unwrap().0, 1);
        assert_eq!(res.unwrap().1, Operator::Add);
    }

    #[test]
    fn find_operator_03() {
        let tokens = tokenize("x-1+0.5").unwrap();
        let res = find_operator(&tokens);
        assert_eq!(res.is_some(), true);
        assert_eq!(res.unwrap().0, 1);
        assert_eq!(res.unwrap().1, Operator::Substract);
    }

    #[test]
    fn find_operator_04() {
        let tokens = tokenize("x+1-0.5").unwrap();
        let res = find_operator(&tokens);
        assert_eq!(res.is_some(), true);
        assert_eq!(res.unwrap().0, 3);
        assert_eq!(res.unwrap().1, Operator::Substract);
    }

    #[test]
    fn find_operator_05() {
        let tokens = tokenize("x+(1-0.5)").unwrap();
        let res = find_operator(&tokens);
        assert_eq!(res.is_some(), true);
        assert_eq!(res.unwrap().0, 1);
        assert_eq!(res.unwrap().1, Operator::Add);
    }

    #[test]
    fn find_operator_06() {
        let tokens = tokenize("(x+5)+(x-0.5)").unwrap();
        let res = find_operator(&tokens);
        assert_eq!(res.is_some(), true);
        assert_eq!(res.unwrap().0, 5);
        assert_eq!(res.unwrap().1, Operator::Add);
    }

    #[test]
    fn parse_token_1() {
        let exp = parse("x");
        assert_eq!(exp.is_ok(), true);
        let node = exp.unwrap();
        match node.left {
            ExpressionNodeType::VariableOperand(va) => assert_eq!(va, 'x'),
            _ => assert!(false),
        }
        match node.operator {
            Some(op) => assert!(false),
            None => assert!(true),
        }
        match node.right {
            Some(op) => assert!(false),
            None => assert!(true),
        }
    }

    const PI: f64 = 3.1415926535897932;
    const EPSILON: f64 = 0.0000000000000001;
    const TEST_MAX: f64 = 1.0e16;

    #[test]
    fn parse_trigonometric_sin() {
        let exp = parse("sin(x/2)");
        assert_eq!(exp.is_ok(), true);
        let exp = exp.unwrap();
        assert_eq!(exp.generate_expression(), "sin(x/2)");

        let mut res = exp.evaluate("x", 0.0);
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.unwrap(), 0.0);

        res = exp.evaluate("x", PI);
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.unwrap(), 1.0);

        res = exp.evaluate("x", PI / 2.0);
        assert_eq!(res.is_ok(), true);
    }

    // #[test] TODO fix
    fn parse_token_2() {
        let exp = parse("3x");
        assert_eq!(exp.is_ok(), true);
        let node = exp.unwrap();
        match node.left {
            ExpressionNodeType::NodeOperand(node2) => {
                let node = node2.as_ref();
                match &node.left {
                    ExpressionNodeType::ValueOperand(va) => assert_eq!(*va, 3.0),
                    _ => assert!(false),
                }
                match &node.operator {
                    Some(op) => match op {
                        Operator::Multiply => assert!(true),
                        _ => assert!(false),
                    },
                    None => assert!(false),
                }
                match &node.right {
                    Some(op) => match op {
                        ExpressionNodeType::VariableOperand(va) => assert_eq!(*va, 'x'),
                        _ => assert!(false),
                    },
                    None => assert!(false),
                }
            }
            _ => assert!(false),
        }
        match node.operator {
            Some(op) => assert!(false),
            None => assert!(true),
        }
        match node.right {
            Some(op) => assert!(false),
            None => assert!(true),
        }
    }
}
