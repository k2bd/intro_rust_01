fn main() {
    let a = String::from("Hello");
    let mut b = String::from("Hello");

    println!("{}, World!", a);

    b.push_str(", World!");
    println!("{}", b);

    //a.push_str(", World!");  // Not allowed! a is immutable
}