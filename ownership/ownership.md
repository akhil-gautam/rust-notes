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

- non-stack data is passed to functions by **move** while the stack-data is copied

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

- same goes for functions that return values, the return value is `moved` into the caller

## references and borrowing

- instead of `moving` values, we can pass it's `reference` to a function and can continue using the value in the caller
- `references` are pointers guaranteed to point to a valid value
- reference of a vlaue is sent using `&` operator and the receiver stores it in a pointer of type `&type`

```rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

- process of creating a reference and using it in other function is called borrowing
- by default borrowers are not allowed to mutate the values

## mutable references
- mutable references are created using `&mut <variable>` and accepted in `&mut <data_type>`
- only one mutable reference can created in a single scope
- immutable reference and then mutable reference is not allowed if immutable one is still in scope

## slice type
- if want reference to a specific part of string, we can use slice reference `&<variable>[start_index..end_index]`
- 