pub mod MSELoss;

pub trait loss_function {
    fn get_loss_and_backward(&mut self, _x: f64) -> (f64, f64) {
        todo!()
    }
}
