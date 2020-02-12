fn main() {
    let k;

    {

        let a = String::from("Hello!");

        k = &a;

        println!("{}, World!", k);
    }

    //println!("{}, World!", k);  // Not allowed!
}
