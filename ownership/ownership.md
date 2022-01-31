## ownership & memory allocation

```rust
fn main() {
    let s1 = String::from("hello");
    // s1 holds data on stack
    // the data consists of 3 parts:
    // 1. pointer: pointer to the memory on heap that holds that actual string,
    // 2. length: current length of the string content,
    //3. capacity: total memory in bytes that String recieved from the allocator
    let s2 = s1;

    println!("{}, world!", s1);
    println!("{}, world!", s2);
}
```

- we will get error in the above code be to ensure memory safety, Rust's ownership principal doesn't point both `s1` and `s2` to the same memory block, instead makes `s1` invalid after `s2 = s1` and just points `s2` to the block pointed by `s1` earlier.
- `s1 = s2` just copies the data `s1` is holding on `stack` and this whole process is called **move**
- to explicitly clone `s1` to `s2`, we can invoke `clone` method

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}
// s1 = hello, s2 = hello
```

- when dealing with stack-only data(data that has `Copy` trait) or scalar data, when we do `x = y`, data is automatically copied on stack

```rust
fn main() {
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);
}
// x = 5, y = 5
```

## ownership & functions

- non-stack data is passed to functions is by **move** and the stack-data is copied

```rust
fn main() {
    let s = String::from("hello");

    takes_ownership(s);
    // s is invalid here, because it has been moved

    let x = 5;
    makes_copy(x);
    // x is valid here, because it is a copy
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}
```
