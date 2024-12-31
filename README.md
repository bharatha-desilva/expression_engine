# Expression Engine Project

#### Usage 

```rust
use expression_engine::{parse, ExpressionNode, ExpressionNodeType, Operator};
use std::f32::consts::PI;

const EPSILON: f64 = 0.000000001;

let test = |exp_str: &str, input: f64| -> f64 {
    parse(exp_str).unwrap().evaluate("x", input).unwrap()
};

assert_eq!(test("x", 1.5), 1.5);
assert_eq!(test("x^2", 1.5), 2.25);
assert_eq!(test("x^2 + x^3", 1.5), 5.625);
assert!((test("sin x", PI as f64 / 2.0) - 1.0).abs() < EPSILON);
```