pub(crate) fn main() {
    println!("Hello, world!");

    another_function(5);
    plus_one(5);
}

fn another_function(x: i32) {
    println!("The value of x is {}", x);
}
fn plus_one(x: i32) -> i32 {
    x + 1
}
