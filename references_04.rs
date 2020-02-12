fn main() {
    let mut a = 5;

    // Brackets create scopes
    {
        let i = &a;
        let j = &a;

        let k = &mut a;  // NOT ALLOWED
        
        *k +=1;
        
        println!("i+j={}", i + j);
        println!("k={}", k);
    }

    println!("a={}", a);
}