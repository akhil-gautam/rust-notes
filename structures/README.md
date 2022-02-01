- a structure is used to group data with a meaningful name
- it is similar to tuple but it has a `name` and it doesn't need parameters to be in order
- structure is created using `struct` keyword followed by name of structure

```rust
struct Reactangle {
  height: u32,
  width: u32
}

let rect1 = Reactangle {height: 20, width: 40}
```

- methods can be defined on struct using `impl` implementations and these are called associated methods

```rust
impl Rectangle {
  fn area(&self) -> u32 {
    self.height * self.width
  }
}
```

- associated functions can also be created that aren't dependent on `self`

```rust
impl Rectangle {
    fn square(size: u32) -> Rectangle { // here it is used to create a new instance of Reactangle
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let sq = Rectangle::square(3); // these kind of functions are invoked using :: operator
}
```

- there can be multiple `impl` blocks for a `struct`
