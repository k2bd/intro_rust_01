fn main() {
    let s1 = String::from("Hello");
    let mut s2 = s1;

    s2.push_str(", World!");

    println!("{}", s2);
    //println!("{}, world!", s1);
}