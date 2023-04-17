use crate::function::Function;
use std::fmt::{self, Display};

#[derive(Debug)]
pub struct LinearFunction {
    multiplier: f64,
    data: Option<f64>,
    back: Option<f64>,
}

impl LinearFunction {
    pub fn new() -> Self {
        Self {
            multiplier: 3.0,
            data: None,
            back: None,
        }
    }

    pub fn get_multiplier(&self) -> f64 {
        self.multiplier
    }
}

impl Display for LinearFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "multiplier={}, data={:?}, back={:?}",
            self.multiplier, self.data, self.back
        )
    }
}

impl Function for LinearFunction {
    fn forward(&mut self, _x: f64) -> f64 {
        self.data = Some(_x);
        _x * self.multiplier
    }

    fn backward(&mut self, _dy: f64) -> f64 {
        assert!(self.data.is_some());

        self.back = Some(_dy * self.data.unwrap());
        _dy * self.multiplier
    }

    fn backward_step(&mut self, alpha: f64) {
        assert!(self.back.is_some());
        let b = self.back.unwrap();
        self.multiplier -= alpha * b;
        self.back = None;
        self.data = None;
    }
}
