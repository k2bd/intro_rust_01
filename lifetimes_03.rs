fn main() {
    let a = 5;

    let c;
    {
        let b = 4;

        c = largest_number(&a, &b);
    }
    println!("{}", c);
}

fn largest_number<'a>(x: &'a usize, y: &'a usize) -> &'a usize {
    if x > y {
        x
    } else {
        y
    }
}