fn main() {
    println!("Smallest positive number divisible by all numbers from 1-20");

    'outer: for x in (2520..).step_by(2) {
        let mut count = 0;
        for z in 1..21 {
            if x % z == 0 {
                count += 1;
            }

            if count == 20 {
                println!("Smallest number {x}");
                break 'outer;
            }
        }
    }
}
