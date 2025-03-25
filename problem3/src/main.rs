fn main() {
    println!("Problem 3: largest prime factor");

    let mut target: i64 = 600851475143;
    let sqroot: i64 = target / 2;

    println!("rounded {sqroot}");

    for x in (3..sqroot).step_by(2) {
        if target % x == 0 {
            target = target / x;
            println!("Prime factor: {x}");
        }
        if target == 1 {
            break;
        }
    }
}
