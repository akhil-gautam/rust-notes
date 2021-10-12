## Scalar data types

### integer

- they can be signed(`i`) or unsigned(`u`): example `let num: i64 = 34`
- integer defaults to i32

### float

- `f32` and `f64` are the two floating types in Rust

### boolean

- `true` and `false` are the two boolean(`bool`) values in Rust

### character

- `char` is used to represent character type
- `char` is 4 bytes in size

## Compound data types

### tuple

- tuples have a fixed length, example: `let tup: (i32, f64, u8) = (500, 6.4, 1);`
- values can be of different data_types
- index in tuple starts from 0
- tuples' values can be destructured as following:

```rust
fn main() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {}", y);
}
// The value of y is: 6.4
```

- values can also be destructured using `.` with the index of the value:

```rust
fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
}
```

### array

- array is also a collection but values have to be of the same data_type
- arrays also have a fixed length
- array elements are accessed using `arr[index]`

```rust
fn main() {
    // this declares an array of size 5 where elements are of type i32
    let a: [i32; 5] = [1, 2, 3, 4, 5];
}


fn main() {
    // declare an array of size 5 where each fo the values are 3
    let a = [3; 5];
    println!("{}", a)

    // => [3, 3, 3, 3, 3]
}
```
