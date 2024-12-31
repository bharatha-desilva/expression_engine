use std::fmt;

#[derive(Clone)]
pub enum TrigonometricFunction {
    Sin,
    Cos,
    Tan,
    Sec,
    Cosec,
    Cot,
}

impl TrigonometricFunction {
    pub fn from(token_str: &str) -> Option<TrigonometricFunction> {
        match token_str {
            "sin" => Some(TrigonometricFunction::Sin),
            "cos" => Some(TrigonometricFunction::Cos),
            "tan" => Some(TrigonometricFunction::Tan),
            "sec" => Some(TrigonometricFunction::Sec),
            "cosec" => Some(TrigonometricFunction::Cosec),
            "cot" => Some(TrigonometricFunction::Cot),
            _ => None,
        }
    }

    pub fn evaluate(&self, value: f64) -> f64 {
        match self {
            TrigonometricFunction::Sin => value.sin(),
            TrigonometricFunction::Cos => value.cos(),
            TrigonometricFunction::Tan => value.tan(),
            TrigonometricFunction::Sec => 1.0 / value.cos(),
            TrigonometricFunction::Cosec => 1.0 / value.sin(),
            TrigonometricFunction::Cot => 1.0 / value.tan(),
        }
    }
}

impl fmt::Display for TrigonometricFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            TrigonometricFunction::Sin => write!(f, "sin")?,
            TrigonometricFunction::Cos => write!(f, "cos")?,
            TrigonometricFunction::Tan => write!(f, "tan")?,
            TrigonometricFunction::Sec => write!(f, "sec")?,
            TrigonometricFunction::Cosec => write!(f, "cosec")?,
            TrigonometricFunction::Cot => write!(f, "cot")?,
        }
        Ok(())
    }
}
