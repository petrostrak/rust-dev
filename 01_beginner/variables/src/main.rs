fn main() {
    // creation
    let a: i16 = 5;

    // mutability
    let mut b: i32 = 10;
    b = 15;

    // shadowing
    let c: i16 = 25;
    let c: i16 = 30;
    println!("c is {c}");

    // scope
    let d = 30;
    {
        let d = 40;
        println!("d is {d}");
    }

    println!("d is {d}");
}
