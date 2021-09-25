fn main() {
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
}