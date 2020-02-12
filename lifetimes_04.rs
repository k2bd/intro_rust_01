fn main() {
    let a = 5;

    let c;
    {
        let b = 4;

        c = largest_number(&a, &b);
        println!("{}", c);
    }
    //println!("{}", c);  // Not allowed!
}

fn largest_number<'t>(x: &'t usize, y: &'t usize) -> &'t usize {
    if x > y {
        x
    } else {
        y
    }
}