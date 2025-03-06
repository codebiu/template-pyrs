pub struct Calculator;

impl Calculator {
    pub fn new() -> Self {
        Calculator
    }

    pub fn add(&self, a: f64, b: f64) -> f64 {
        a + b
    }

    pub fn subtract(&self, a: f64, b: f64) -> f64 {
        a - b
    }

    pub fn multiply(&self, a: f64, b: f64) -> f64 {
        a * b
    }

    pub fn divide(&self, a: f64, b: f64) -> Result<f64, &'static str> {
        if b == 0.0 {
            Err("除数不能为零")
        } else {
            Ok(a / b)
        }
    }
}

pub mod ffi;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculator() {
        let calc = Calculator::new();
        assert_eq!(calc.add(2.0, 3.0), 5.0);
        assert_eq!(calc.subtract(5.0, 3.0), 2.0);
        assert_eq!(calc.multiply(4.0, 2.0), 8.0);
        assert_eq!(calc.divide(6.0, 2.0).unwrap(), 3.0);
        assert!(calc.divide(1.0, 0.0).is_err());
    }
}
