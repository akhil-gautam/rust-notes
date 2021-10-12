// fn main() {
//     println!("Hello, world!");

//     another_function();
// }

// fn another_function() {
//     println!("Another function.");
// }

fn five() -> i32 { // return type of a function is written after ->
    5
}

fn main() {
    let x = five();

    println!("The value of x is: {}", x);
}