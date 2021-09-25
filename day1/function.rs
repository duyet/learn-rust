fn sum(a: i8, b: i8) -> i8 {
    a + b
}

fn main() {
    println!("sum(1, 2) = {}", sum(1, 2));

    let s: fn(i8, i8) -> i8 = sum;
    println!("s(1, 2) = {}", s(1, 2));

    let sum_closures_1 = |a: i8, b: i8| -> i8 {
        a + b
    };
    println!("sum_closures_1(1, 2) = {}", sum_closures_1(1, 2));

    // closures without type
    let sum_closures_2 = |a, b| { a+ b };
    println!("sum_closures_2(1, 2) = {}", sum_closures_2(1, 2));

    // { } are optional for single-lined closures
    let sum_closures_3 = |a, b| a + b;
    println!("sum_closures_3(1, 2) = {}", sum_closures_3(1, 2));

    // creating and calling together
    let sum_closures_4 = |a, b| -> i32 {a + b}(1, 2);
    println!("sum_closures_4 = {}", sum_closures_4);
}