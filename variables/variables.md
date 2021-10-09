## variables

- when we declare a variable using `let` keyword, it is immutable by default
- to declare a mutable variable, `mut` keyword is used

## constants

- `const` keyword is used to declare them
- constants are always immutable and must be annotated with data-type
- constants can be declared anywhere, even in global scope
- constants can only be used for a constant expression, never used for values computed at runtime
- naming convention for constant is all uppercase with underscore in case of multi-word names
- constants are alive till the program runs, whithin the scope it is declared in when it is not a global variable

## shadowing

- when a variable is redeclared with the same name

```rust
let x = 4

let x = x * 2   // the variable x is shadowed here

println!("The value of x is: {}.", x)
// => The value of x is 8.
```

- while shadowing we can change the type of variable
