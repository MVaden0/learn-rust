fn main() {
    // this is a mutable variable
    let mut x = 5;

    println!("The value of x is {x}");

    x = 6;

    println!("The value of x is {x}");

    // constants are always immutable and have global scope
    const SECONDS: u32 = 60;

    // variables can be 'shadowed' as such:
    let y = 1;
    let y = 0;
}
