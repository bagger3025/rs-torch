use super::loss_function;

pub struct MSELoss {
    target: f64,
}

impl MSELoss {
    pub fn new(target: f64) -> Self {
        Self { target }
    }
}

impl loss_function for MSELoss {
    fn get_loss_and_backward(&mut self, _x: f64) -> (f64, f64) {
        let base = _x - self.target;
        (base * base, 2.0 * base)
    }
}
