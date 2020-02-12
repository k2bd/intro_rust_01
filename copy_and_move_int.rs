fn main() {
    let i1 = 5;
    let mut i2 = i1;

    i2 += 1;
    
    println!("i2 is: {}", i2);
    println!("i1 is: {}", i1);  // Allowed?
}