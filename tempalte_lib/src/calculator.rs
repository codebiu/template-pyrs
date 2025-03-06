// public 默认 private
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