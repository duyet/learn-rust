fn main() {
    let a0: [i32; 3] = [1, 2, 3];
    let c1: &[i32] = &a0;
    println!("c1 = {:?}", c1);

    let c2 = &a0[0..2];
    println!("c2 = {:?}", c2);

    let a1 = "hello";
    let c3 = &a1[..2];
    println!("c3 = {:?}", c3);

    let c4: &str = "ok";
    println!("c4 = {:?}", &c4[1..2]);
}