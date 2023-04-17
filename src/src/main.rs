mod function;
mod loss;
use function::linear::LinearFunction;
use function::Function;
use loss::loss_function;
use loss::MSELoss::MSELoss;

fn main() {
    let alpha = 0.001;
    let mut my_function = LinearFunction::new();
    let my_data = 3.0;
    let target = 10.0;
    println!("myFunction={my_function}, my_data={my_data}");
    loop {
        let mul = my_function.get_multiplier();
        let y = my_function.forward(my_data);
        let mut loss_calculator = MSELoss::new(target);
        let (_, dloss) = loss_calculator.get_loss_and_backward(y);
        let dx = my_function.backward(dloss);
        println!("y={y}, dloss={dloss}, dx={dx}");
        my_function.backward_step(alpha);
        if my_function.get_multiplier() - mul < 1e-15 {
            break;
        }
    }

    println!("Result myFunction={my_function}");
}
