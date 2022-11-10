fn main() {
    another_function(23);

    let x = five();

    println!("the value of x is {x}")
}

fn another_function(x: i32) {
    println!("the value of x is {x}");
}

fn five() -> i32 {
    5
}
