# Day 1

```bash
cd day1
rustc main.rs
./main
```

`println!()` prints text to the console and its `!` indicates that it’s a [macro](https://doc.rust-lang.org/book/ch19-06-macros.html) rather than a function.

- Macros are a way of writing code that writes other code, which is known as *metaprogramming*.
- Macros have some additional powers that functions don’t. 
- We can call `println!("hello")` with one argument or `println!("hello {}", name)` with two arguments. 

## Cargo

Cargo is Rust's built-in package manager.

- Create a new project: `cargo new`
- Create a new project in an existing directory: `cargo init`
- Build the project: `cargo build`
- Run the project: `cargo run`

## crates.io

crates.io is Rust's official crate registry.

## Crate

A crate is a package, which can be shared via crates.io. A crate can produce an executable or a library. In other words, it can be a binary crate or a library crate.

```
cargo new duyet_bin --bin
# or: cargo new duyet_lib --lib
```

It's will create a new crate package:

```
duyet_bin
├── Cargo.toml
└── src
    └── main.rs

1 directory, 2 files
```

```
duyet_lib
├── Cargo.toml
└── src
    └── lib.rs

1 directory, 2 files
```

## Variables

Variables are immutable by default. We call them Variable bindings.

```rs
let a: i32;
a = 5; // assignment
a = 10;

// error[E0384]: cannot assign twice to immutable variable `a`
//    |
// 1  |     let z: i32;
//    |         - help: consider making this binding mutable: `mut a`
// 2  |     a = 5;
//    |     ----- first assignment to `a`
// 3  |     a = 10;
//    |     ^^^^^^^ cannot assign twice to immutable variable
```

To make them mutable, the `mut` keyword is used.

```rs
let mut a: i32 = 5;
a = 6;
a = 10; 
```

```rs
let (x, y) = (1, 2); // x = 1 and y = 2

let z = { x + y }; // z = 3

let z = {
    let x = 1;
    let y = 2;

    x + y
}; // z = 3
```

- The default integer type is `i32` and the default float type is `f64`.
- For numeric types, we can append the data type directly to the value as the suffix. (e.g. `let a = 5i8`).
- Use `_`: `let a = 1_000_000`, `let pi = 3.141_123`. `10000_0000` is also valid.

## Naming convention

- The naming convention for the variable bindings is using the *snake_case*.
- Constants and statics, we should follow the *SCREAMING_SNAKE_CASE*.

```rs
let this_is_var: i32 = 100;
const PI: f64 = 3.14;
```

## Variable Shadowing

Rust allows us to redeclare the same variable with a different data type and/ or with a different mutability setting.

```rs
let x: f64 = -20.48; // float
let x: i64 = x.floor() as i64; // int
println!("{}", x); // -21
```

## Function

We can skip the `return` keyword at the last expression (like Ruby), no ending `;`.

```rs
fn sum(a: i8, b: i8) -> i8 {
    a + b
}

fn main() {
    println!("{}", sum(1, 2));
}
```

## Function pointer

```rs
let s: fn(i8, i8) -> i8 = sum;
s(1, 2);
```

## Closures (lambda function)

The sum function above can be writed as closures:

```rs
let sum_closures_1 = |a: i8, b: i8| -> i8 {
    a + b
};
println!("sum_closures_1(1, 2) = {}", sum_closures_1(1, 2));

// closures without type
let sum_closures_2 = |a, b| { a + b };
println!("sum_closures_2(1, 2) = {}", sum_closures_2(1, 2));

// { } are optional for single-lined closures
let sum_closures_3 = |a, b| a + b;
println!("sum_closures_3(1, 2) = {}", sum_closures_3(1, 2));

// creating and calling together
let sum_closures_4 = |a, b| -> i32 {a + b}(1, 2);
println!("sum_closures_4 = {}", sum_closures_4);
```

## Array

```rs
let a0 = [1, 2, 3];
println!("a0 = {:?}", a0);
println!("a0 = {:#?}", a0);

let a1: [i32; 3] = [1, 2, 3];
println!("a1 = {:?}", a1);

let a2 = [0; 5];
println!("a2 = {:?}", a2);

let mut a3 = [0; 5];
a3[1] = 1;
println!("a3 = {:?}", a3);
```

## Tuple

To change an element’s value, the new value should have the same data type of previous value.


```rs
let a0 = (1, true , "a");
println!("a0 = {:?}", a0);
println!("a0 = {:#?}", a0);

let mut a1 = (1, true , "a");
a1.1 = false;
println!("a1 = {:?}", a1);

let (z, _, zz) = a1;
println!("z = {}\nzz = {}", z, zz);
```

## Slice

Dynamically-sized reference to another data structure

```rs
let a: [i32; 4] = [1, 2, 3, 4]; // Parent Array

let b: &[i32] = &a; // Slicing whole array
let c = &a[0..4]; // From 0th position to 4th(excluding)
let d = &a[..]; // Slicing whole array

let e = &a[1..3]; // [2, 3]
let f = &a[1..]; // [2, 3, 4]
let g = &a[..3]; // [1, 2, 3]

```
