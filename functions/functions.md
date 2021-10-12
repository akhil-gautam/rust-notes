- function definitions start with `fn` keyword
- functions can accept parameters.

```rust
fn main() {
    another_function(5);
}

fn another_function(x: i32) { // data type of parameters is required
    println!("The value of x is: {}", x);
}
```

- functions can explicitly return values using `return` keyword
- if the last line of the function is an expression, it will be evaluated and returned implicitly

```rust
fn five() -> i32 { // return type of a function is written after ->
    5
}

fn main() {
    let x = five();

    println!("The value of x is: {}", x);
}
```
