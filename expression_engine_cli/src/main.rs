use std::env::{self, Args};

use expression_engine::{parse, ExpressionNode, ExpressionNodeType, Operator};

fn main() {
    let args: Vec<String> = env::args().collect();

    let exp = &args[1];

    match parse(exp) {
        Ok(exp) => {
            println!("Expression {}", exp);
            let res = exp.evaluate("x", args[2].parse::<f64>().unwrap());
            match res {
                Ok(value) => println!("Evaluation Result {}", value),
                Err(_) => todo!(),
            }
        }
        Err(error) => match error.kind {
            expression_engine::ErrorKind::Empty => println!("Empty expression"),
            expression_engine::ErrorKind::InvalidOpenCloseParantheses => {
                println!("Incorrect number of open/close parantheses")
            }
            expression_engine::ErrorKind::InvalidNumberParsed => {
                println!("Failed to parse number(s) in expression")
            }
            expression_engine::ErrorKind::InvalidExpression => println!("Expression is invalid"),
        },
    }
}
