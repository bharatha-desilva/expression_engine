use std::fmt;

#[derive(Debug, Eq, PartialEq, PartialOrd, Clone, Copy)]
pub enum Operator {
    Substract,
    Add,
    Divide,
    Multiply,
    Modulus,
    Power,
}

impl Operator {
    pub fn from(op: &str) -> Option<Operator> {
        match op {
            "+" => Some(Operator::Add),
            "-" => Some(Operator::Substract),
            "*" => Some(Operator::Multiply),
            "/" => Some(Operator::Divide),
            "%" => Some(Operator::Modulus),
            "^" => Some(Operator::Power),
            _ => None,
        }
    }

    pub fn value_in_operator(&self) -> char {
        match self {
            Operator::Add => '+',
            Operator::Substract => '-',
            Operator::Multiply => '*',
            Operator::Divide => '/',
            Operator::Modulus => '%',
            Operator::Power => '^',
        }
    }
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value_in_operator())?;
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compare_in_operatoe() {
        assert_eq!(Operator::Add > Operator::Substract, true);
        assert_eq!(Operator::Substract > Operator::Add, false);
        assert_eq!(Operator::Multiply > Operator::Add, true);
        assert_eq!(Operator::Add > Operator::Multiply, false);
        assert_eq!(Operator::Multiply > Operator::Divide, true);
        assert_eq!(Operator::Divide > Operator::Multiply, false);
        assert_eq!(Operator::Modulus > Operator::Multiply, true);
        assert_eq!(Operator::Multiply > Operator::Modulus, false);
        assert_eq!(Operator::Power > Operator::Modulus, true);
        assert_eq!(Operator::Modulus > Operator::Power, false);
    }

    #[test]
    fn values_in_operatoe() {
        assert_eq!(Operator::Add.value_in_operator(), '+');
        assert_eq!(Operator::Substract.value_in_operator(), '-');
        assert_eq!(Operator::Multiply.value_in_operator(), '*');
        assert_eq!(Operator::Divide.value_in_operator(), '/');
        assert_eq!(Operator::Modulus.value_in_operator(), '%');
        assert_eq!(Operator::Power.value_in_operator(), '^');
    }

    #[test]
    fn operator_from() {
        assert_eq!(
            Operator::from("+").is_some_and(|op| op == Operator::Add),
            true
        );
        assert_eq!(
            Operator::from("-").is_some_and(|op| op == Operator::Substract),
            true
        );
        assert_eq!(
            Operator::from("*").is_some_and(|op| op == Operator::Multiply),
            true
        );
        assert_eq!(
            Operator::from("/").is_some_and(|op| op == Operator::Divide),
            true
        );
        assert_eq!(
            Operator::from("^").is_some_and(|op| op == Operator::Power),
            true
        );
        assert_eq!(
            Operator::from("%").is_some_and(|op| op == Operator::Modulus),
            true
        );
        assert_eq!(Operator::from("?").is_none(), true);
    }
}
