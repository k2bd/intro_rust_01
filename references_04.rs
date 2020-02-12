fn main() {
    let mut a = 5;

    // Brackets create scopes
    {
        let i = &a;
        let j = &a;

        let k = &mut a;  // Not allowed!

        *k +=1;

        println!("i+j={}", i + j);
        println!("k={}", k);
    }

    println!("a={}", a);
}