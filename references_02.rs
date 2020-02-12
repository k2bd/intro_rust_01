fn main() {
    let mut a = 5;

    // Brackets create scopes
    {
        let i = &a;
        let j = &a;

        println!("i+j={}", i + j);
    }
    {
        let k = &mut a;
        
        *k +=1;
        
        println!("k={}", k);
    }

    println!("a={}", a);
}