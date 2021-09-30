# Day 2

## Control flows

Normal

```rs
if a < 50 {
    // ...
} else {
    // ...
}
```

Using with `let` statement.

```rs
let age: u8 = 13;
let is_below_eighteen = if age < 18 { true } else { false }; // true
```

⭐️ Return data type should be the same on each block when using this as an expression.


## Match

This feature is so cool, i love it.

```rs
let a = 10;
let size = match a {
    16 => "S",
    17 | 18 => "M", // 17 or 18
    19..=21 => "L", // (19, 20, 21)
    22 => "XL",
    _ => "NA",
};
println!("{}", size);
```

Match with two variables:

```rs
let a = 5;
let b = 10;

let res = match (a, b) {
    (50, 50) => "ok",
    (50, _) => "left",
    (_, 50) => "right",
    (x, y) if x > 25 && y > 25 => "25",
    (_, _) => "NA",
};
println!("{}", res);
```

## Loop

Just like `white(true)`

```rs
loop {
    println!("Loop forever!");
}
```

Nested loop with label, how can you break the parent loop like this:

```rs
// Outer break
let mut b1 = 1;

'outer_loop: loop {
    //set label outer_loop
    let mut b2 = 1;

    'inner_loop: loop {
        println!("Current Value : [{}][{}]", b1, b2);

        if b1 == 2 && b2 == 2 {
            break 'outer_loop; // kill outer_loop
        } else if b2 == 5 {
            break 'inner_loop;
        }

        b2 += 1;
    }

    b1 += 1;
}
```

## While

While can have a label, too.

```rs
// Outer break
let mut c1 = 1;

'outer_while: while c1 < 6 { //set label outer_while
    let mut c2 = 1;

    'inner_while: while c2 < 6 {
        println!("Current Value : [{}][{}]", c1, c2);
        if c1 == 2 && c2 == 2 { break 'outer_while; } //kill outer_while
        c2 += 1;
    }

    c1 += 1;
}
```

## for

```rs
for i in 1..10 { // In other languages, for (i = 0; i < 10; i++)
    println!("{}", i);
}

for i in 1..=10 { // In other languages, for (i = 1; i <= 10; i++)
  println!("{}", i);
}
```

`for` can have a label, too.

Loop in arrays/vectors:

```rs
let group: [&str; 4] = ["a", "b", "c", "d"];

for n in group.iter() {
    println!("{}", n);
}
```

## Vectors

A vector is kind of a re-sizable array but all elements must be in the same type.

```rs
let mut a = Vec::new();
let mut b = vec![];
let mut c = vec![1, 2, 3];
let mut d: Vec<i8> = Vec::new();
let mut e: Vec<i8> = vec![1i8, 2, 3];
let mut f = vec![1; 10]; // 1,1,1,1,1,1....
```