fn main() {
    let s1 = String::from("Hello!");

    let len = calculate_length(s1);

    println!("The length of the string is {}", len);

    // What about this, more informative print statement?
    //println!("The length of '{}' is {}", s1, len);
}

fn calculate_length(s: String) -> usize {
    s.len()
}
