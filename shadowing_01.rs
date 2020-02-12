fn main() {
    let some_variable = 5;

    {
        let some_variable = "world!";

        println!("Hello, {}", some_variable);
    }
    println!("Hello, {}", some_variable);
}
