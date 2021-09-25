/// Outer comment
#[doc = "outer comment"]

fn main() {
    println!("duyet");
    println!("hello {}", "duyet");
    println!("hello {name}", name="duyet");

    // The format! macro is used to store the formatted string
    let x = format!("{}, {}!", "Hello", "world");
    println!("{}", x); // Hello, world!

    println!();

    /** This is inline comment, document the next item */
    const A: i32 = 1;
    println!("{}", A);
    println!();

    // var shadowing
    let x: f64 = -20.48; // float
    let x: i64 = x.floor() as i64; // int
    println!("{}", x); // -21

    let x: &str = "hello";
    let x: String = x.to_string();
    println!("{}", x);

    let mut z: i32;
    z = 100;
    z = 200;
    println!("let mut z: i32 = {}", z);

    let (a, b) = (1, 2);
    let c = { a + b };
    println!("c = {}", c);

    let z = {
        let a = 1;
        let b = 2;
        a + b
    };

    println!("z = {}", z);
}