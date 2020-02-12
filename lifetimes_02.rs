fn main() {
    let a = 5;
    let b = 6;

    println!("The largest of a and b is {}",largest_number(&a, &b));
    println!("The sum of a and b is {}",a + b);
}

fn largest_number<'a>(x: &'a usize, y: &'a usize) -> &'a usize {
    if x > y {
        x
    } else {
        y
    }
}