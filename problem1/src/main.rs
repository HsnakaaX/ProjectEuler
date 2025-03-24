fn main() {
    println!("Multiples of 3 or 5 below 1000");
    let mut sum = 0;

    for x in 1..1000 {
        if x % 3 == 0 || x % 5 == 0 {
            sum += x;
        }
    }
    println!("Sum {sum}")
}
