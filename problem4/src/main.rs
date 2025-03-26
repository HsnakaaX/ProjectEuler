fn main() {
    println!("Problem 4: largest palindrome made by the product of 2 three digit numbers");

    let initial_value = 100;
    let end_value = 1000;
    let mut largest_palindrome = 0;

    for x in (initial_value..end_value).rev() {
        for z in (initial_value..end_value).rev() {
            let product = x * z;
            let palin = get_palindrome(product);
            if palin == product {
                if product > largest_palindrome {
                    largest_palindrome = product;
                }
            }
        }
    }

    println!("Largest Palindrome {largest_palindrome}");
}

fn get_palindrome(x1: i32) -> i32 {
    let mut new_pal = 0;
    let mut y = x1;
    while y > 0 {
        let rem = y % 10;
        new_pal = (new_pal * 10) + rem;
        y = y / 10;
    }
    return new_pal;
}
