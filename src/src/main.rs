mod function;
use function::linear::LinearFunction;
use function::Function;

fn main() {
    let alpha = 0.001;
    let mut my_function = LinearFunction::new();
    let my_data = 3.0;
    println!("First initialization:");
    println!("myFunction={my_function}, my_data={my_data}");
    loop {
        let mul = my_function.get_multiplier();
        let y = my_function.forward(my_data);
        println!("y={y}");
        let _loss = (y - 10.0) * (y - 10.0);
        let dloss = 2.0 * (y - 10.0);
        println!("dloss={dloss}");
        let dx = my_function.backward(dloss);
        println!("{dx}");
        my_function.backward_step(alpha);
        if my_function.get_multiplier() - mul < 1e-15 {
            break;
        }
    }

    println!("Result myFunction={my_function}");
}
