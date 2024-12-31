use crate::operator::Operator;
use crate::trigonometric::TrigonometricFunction;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvalError {
    pub kind: EvalErrorKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EvalErrorKind {
    NoSubstitute,
    InvalidOpenCloseParantheses,
    InvalidNumberParsed,
    InvalidExpression,
}

#[derive(Clone)]
pub enum ExpressionNodeType {
    ValueOperand(f64),
    VariableOperand(char),
    TrigonometricOperand((TrigonometricFunction, Box<ExpressionNode>)),
    NodeOperand(Box<ExpressionNode>),
}

impl ExpressionNodeType {
    fn evaluate(&self, variable: &str, substitute: f64) -> Result<f64, EvalError> {
        Ok(match &self {
            ExpressionNodeType::ValueOperand(value) => *value,
            ExpressionNodeType::VariableOperand(variable) => substitute, //TODO the validation to check if it's exact variable substitution
            ExpressionNodeType::NodeOperand(expression_node) => {
                expression_node.as_ref().evaluate(variable, substitute)?
            }
            ExpressionNodeType::TrigonometricOperand((trig_fn, expression_node)) => {
                trig_fn.evaluate(expression_node.as_ref().evaluate(variable, substitute)?)
            }
        })
    }
}

impl fmt::Display for ExpressionNodeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            ExpressionNodeType::ValueOperand(value) => write!(f, "{}", value)?,
            ExpressionNodeType::NodeOperand(node) => write!(f, "({})", node.as_ref())?,
            ExpressionNodeType::VariableOperand(var) => write!(f, "{}", var)?,
            ExpressionNodeType::TrigonometricOperand((trig_fn, node)) => {
                write!(f, "{}({})", trig_fn, node.as_ref())?
            }
        }
        Ok(())
    }
}

#[derive(Clone)]
pub struct ExpressionNode {
    pub left: ExpressionNodeType,
    pub operator: Option<Operator>,
    pub right: Option<ExpressionNodeType>,
}

impl ExpressionNode {
    pub fn generate_expression(&self) -> String {
        let mut expression_str = String::new();
        expression_str.push_str(&self.left.to_string());

        match &self.operator {
            Some(op) => expression_str.push_str(&op.to_string()),
            None => (),
        }

        match &self.right {
            Some(node) => expression_str.push_str(&node.to_string()),
            None => (),
        }

        expression_str
    }

    pub fn evaluate(&self, variable: &str, substitute: f64) -> Result<f64, EvalError> {
        let left_val = &self.left.evaluate(variable, substitute)?;
        match &self.right {
            Some(right_node) => match &self.operator {
                Some(operator) => {
                    let right_val = right_node.evaluate(variable, substitute)?;
                    Ok(match operator {
                        Operator::Substract => left_val - right_val,
                        Operator::Add => left_val + right_val,
                        Operator::Divide => left_val / right_val,
                        Operator::Multiply => left_val * right_val,
                        Operator::Modulus => left_val % right_val,
                        Operator::Power => left_val.powf(right_val),
                    })
                }
                None => Ok(left_val * right_node.evaluate(variable, substitute)?), //default no operator means multiplication
            },
            None => Ok(*left_val),
        }
    }
}

impl fmt::Display for ExpressionNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.generate_expression())?;
        Ok(())
    }
}

impl ExpressionNode {
    pub fn new(
        left: ExpressionNodeType,
        operator: Option<Operator>,
        right: Option<ExpressionNodeType>,
    ) -> Self {
        Self {
            left,
            operator,
            right,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse;

    #[test]
    fn evaluate_01() {
        let exp = parse("x");
        assert_eq!(exp.is_ok(), true);
        let res = exp.unwrap().evaluate("x", 0.5);
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.unwrap(), 0.5);
    }

    // #[test] TODO fix
    fn evaluate_02() {
        let exp = parse("3x");
        assert_eq!(exp.is_ok(), true);
        let res = exp.unwrap().evaluate("x", 0.25);
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.unwrap(), 0.75);
    }

    #[test]
    fn evaluate_1() {
        let exp = parse("x+1");
        assert_eq!(exp.is_ok(), true);
        let res = exp.unwrap().evaluate("x", 0.5);
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.unwrap(), 1.5);
    }

    #[test]
    fn evaluate_2() {
        let exp = parse("x-1");
        assert_eq!(exp.is_ok(), true);
        let res = exp.unwrap().evaluate("x", 1.5);
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.unwrap(), 0.5);
    }

    #[test]
    fn evaluate_3() {
        let exp = parse("x*2");
        assert_eq!(exp.is_ok(), true);
        let res = exp.unwrap().evaluate("x", 1.5);
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.unwrap(), 3.0);
    }

    #[test]
    fn evaluate_4() {
        let exp = parse("x^2");
        assert_eq!(exp.is_ok(), true);
        let res = exp.unwrap().evaluate("x", 1.5);
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.unwrap(), 2.25);
    }

    #[test]
    fn evaluate_5() {
        let exp = parse("x%3");
        assert_eq!(exp.is_ok(), true);
        let res = exp.unwrap().evaluate("x", 8.0);
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.unwrap(), 2.0);
    }

    #[test]
    fn evaluate_6() {
        let exp = parse("x+(x^2)");
        assert_eq!(exp.is_ok(), true);
        let res = exp.unwrap().evaluate("x", 1.5);
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.unwrap(), 3.75);
    }

    const PI: f64 = 3.1415926535897932;
    const EPSILON: f64 = 0.0000000000000001;
    const TEST_MAX: f64 = 1.0e16;

    #[test]
    fn evaluate_sin() {
        let exp = ExpressionNode::new(
            ExpressionNodeType::TrigonometricOperand((
                TrigonometricFunction::Sin,
                Box::new(ExpressionNode::new(
                    ExpressionNodeType::VariableOperand('x'),
                    None,
                    None,
                )),
            )),
            None,
            None,
        );
        let mut res = exp.evaluate("x", 0.0);
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.unwrap(), 0.0);

        res = exp.evaluate("x", PI);
        assert_eq!(res.is_ok(), true);
        assert!(res.unwrap() < 0.0000000001);

        res = exp.evaluate("x", PI / 2.0);
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.unwrap(), 1.0);
    }

    #[test]
    fn evaluate_cos() {
        let exp = ExpressionNode::new(
            ExpressionNodeType::TrigonometricOperand((
                TrigonometricFunction::Cos,
                Box::new(ExpressionNode::new(
                    ExpressionNodeType::VariableOperand('x'),
                    None,
                    None,
                )),
            )),
            None,
            None,
        );
        let mut res = exp.evaluate("x", 0.0);
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.unwrap(), 1.0);

        res = exp.evaluate("x", PI);
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.unwrap(), -1.0);

        res = exp.evaluate("x", PI / 2.0);
        assert_eq!(res.is_ok(), true);
        assert!(res.unwrap() < EPSILON);
    }

    #[test]
    fn evaluate_tan() {
        let exp = ExpressionNode::new(
            ExpressionNodeType::TrigonometricOperand((
                TrigonometricFunction::Tan,
                Box::new(ExpressionNode::new(
                    ExpressionNodeType::VariableOperand('x'),
                    None,
                    None,
                )),
            )),
            None,
            None,
        );
        let mut res = exp.evaluate("x", 0.0);
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.unwrap(), 0.0);

        res = exp.evaluate("x", PI);
        assert_eq!(res.is_ok(), true);
        assert!(res.unwrap() < -EPSILON);

        res = exp.evaluate("x", PI / 2.0);
        assert_eq!(res.is_ok(), true);
        assert!(res.unwrap() > TEST_MAX);
    }

    #[test]
    fn generate_expression_1() {
        let exp = ExpressionNode::new(
            ExpressionNodeType::NodeOperand(Box::new(ExpressionNode::new(
                ExpressionNodeType::VariableOperand('x'),
                Some(Operator::Divide),
                Some(ExpressionNodeType::ValueOperand(100.0)),
            ))),
            Some(Operator::Add),
            Some(ExpressionNodeType::ValueOperand(3.5)),
        );
        assert_eq!(exp.generate_expression(), "(x/100)+3.5");
        assert_eq!(format!("{}", exp), "(x/100)+3.5");
    }

    #[test]
    fn generate_expression_2() {
        let exp = ExpressionNode::new(
            ExpressionNodeType::NodeOperand(Box::new(ExpressionNode::new(
                ExpressionNodeType::VariableOperand('x'),
                Some(Operator::Power),
                Some(ExpressionNodeType::ValueOperand(2.0)),
            ))),
            Some(Operator::Add),
            Some(ExpressionNodeType::ValueOperand(3.5)),
        );
        assert_eq!(exp.generate_expression(), "(x^2)+3.5");
        assert_eq!(format!("{}", exp), "(x^2)+3.5");
    }

    #[test]
    fn generate_expression_3() {
        let exp = ExpressionNode::new(
            ExpressionNodeType::NodeOperand(Box::new(ExpressionNode::new(
                ExpressionNodeType::VariableOperand('x'),
                Some(Operator::Power),
                Some(ExpressionNodeType::ValueOperand(2.0)),
            ))),
            Some(Operator::Add),
            Some(ExpressionNodeType::NodeOperand(Box::new(
                ExpressionNode::new(
                    ExpressionNodeType::VariableOperand('x'),
                    Some(Operator::Power),
                    Some(ExpressionNodeType::ValueOperand(3.0)),
                ),
            ))),
        );
        assert_eq!(exp.generate_expression(), "(x^2)+(x^3)");
        assert_eq!(format!("{}", exp), "(x^2)+(x^3)");
    }

    #[test]
    fn generate_expression_sin() {
        let exp = ExpressionNode::new(
            ExpressionNodeType::TrigonometricOperand((
                TrigonometricFunction::Sin,
                Box::new(ExpressionNode::new(
                    ExpressionNodeType::VariableOperand('x'),
                    None,
                    None,
                )),
            )),
            None,
            None,
        );
        assert_eq!(exp.generate_expression(), "sin(x)");
        assert_eq!(format!("{}", exp), "sin(x)");
    }

    #[test]
    fn generate_expression_cos() {
        let exp = ExpressionNode::new(
            ExpressionNodeType::TrigonometricOperand((
                TrigonometricFunction::Cos,
                Box::new(ExpressionNode::new(
                    ExpressionNodeType::VariableOperand('x'),
                    None,
                    None,
                )),
            )),
            None,
            None,
        );
        assert_eq!(exp.generate_expression(), "cos(x)");
        assert_eq!(format!("{}", exp), "cos(x)");
    }

    #[test]
    fn generate_expression_tan() {
        let exp = ExpressionNode::new(
            ExpressionNodeType::TrigonometricOperand((
                TrigonometricFunction::Tan,
                Box::new(ExpressionNode::new(
                    ExpressionNodeType::VariableOperand('x'),
                    Some(Operator::Power),
                    Some(ExpressionNodeType::ValueOperand(3.0)),
                )),
            )),
            None,
            None,
        );
        assert_eq!(exp.generate_expression(), "tan(x^3)");
        assert_eq!(format!("{}", exp), "tan(x^3)");
    }
}
