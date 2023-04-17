pub mod MSELoss;
pub mod linear;

pub trait Function {
    fn forward(&mut self, _x: f64) -> f64 {
        todo!()
    }
    fn backward(&mut self, _dy: f64) -> f64 {
        todo!()
    }
    fn backward_step(&mut self, _alpha: f64) {
        todo!()
    }
}
