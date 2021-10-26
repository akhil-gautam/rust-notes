## conditional flow
- `if`, `else if`, `else` is used for conditional code
```rust
fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}
```
- `if` can also be used on RHS of an assignment
```rust
fn main() {
    let condition = true;

    // data type of both if & else should be same
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);
}
```

## repetation using loops
## `loop`
- `loop` keyword is used to loop over a block
```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}
```
## `while`
- `while` loop is used loop over a block until a given condition true
```rust
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");
}
```
## `for`
- `for` loop is used iterate over a given callection
```rust
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
}
```