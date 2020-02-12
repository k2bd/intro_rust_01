fn main() {
    let mut a = 5;

    // Brackets create scopes
    {
        let i = &a;
        let j = &a;

        //a += 1; // Not allowed! Assignment of borrowed value

        println!("a={}", a);
        println!("i+j={}", i + j);
    }
    {
        let k = &mut a;
        
        *k +=1;
        //a += 1; // Not allowed! Assignment of borrowed value

        println!("a={}", a);  // Not allowed! Why?
        println!("k={}", k);
    }

    println!("a={}", a);
}