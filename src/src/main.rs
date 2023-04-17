use std::fmt::{self, Display};

trait Function {
    fn forward(&mut self, _x: f64) -> f64 {
        todo!("to be implemented")
    }

    fn backward(&mut self, _y: f64) -> f64 {
        todo!();
    }

    fn backward_step(&mut self, alpha: f64);
}

#[derive(Debug)]
struct LinearFunction {
    multiplier: f64,
    data: Option<f64>,
    back: Option<f64>,
}

impl LinearFunction {
    fn new() -> Self {
        LinearFunction {
            multiplier: 3.0,
            data: None,
            back: None,
        }
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

#[derive(Debug)]
struct MyStruct {
    val: i32,
}

impl Display for MyStruct {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({})", self.val)
    }
}

fn main() {
    let alpha = 0.001;
    let mut my_function = LinearFunction::new();
    let my_data = 3.0;
    println!("First initialization:");
    println!("myFunction={my_function}, my_data={my_data}");
    loop {
        let mul = my_function.multiplier;
        let y = my_function.forward(my_data);
        println!("y={y}");
        let _loss = (y - 10.0) * (y - 10.0);
        let dloss = 2.0 * (y - 10.0);
        println!("dloss={dloss}");
        let dx = my_function.backward(dloss);
        println!("{dx}");
        my_function.backward_step(alpha);
        if my_function.multiplier - mul < 1e-15 {
            break;
        }
    }

    println!("Result myFunction={my_function}");
}

#[test]
fn print_mystruct() {
    let my_var = MyStruct { val: 3 };
    println!("{my_var}");
}
