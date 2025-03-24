//use std::env;
fn main() {
    //env::set_var("RUST_BACKTRACE", "full");
    println!("Sum of even Fibonacci series <4 000 000");

    let mut a = 1;
    let mut b = 2;
    let mut sum = 2;
    loop {
        let c = a + b;
        if c >= 4000000 {
            break;
        }
        if c % 2 == 0 {
            println!("Adding {c}");
            sum += c;
        }

        a = b;
        b = c;
    }

    println!("Sum {sum}");
}
