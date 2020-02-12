fn main() {

    {  // Scope begins

        let a = String::from("Hello");  // a owns the string

        println!("{}, World!", a);

    }  // a's scope ends, so the value is dead

}