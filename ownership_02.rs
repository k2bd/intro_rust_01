fn main() {

    let a = String::from("Hello");

    {  // b's Scope begins

        let b = a;  // a transfers ('moves') ownership to b

        println!("{}, World!", b);

        //println!("{}, World!", a);  // Not allowed! (ownership has moved)

    }  // b's scope ends, so the value is dead
    
    //println!("{}, World!", a);  // Still not allowed! (ownership didn't come back)
}
