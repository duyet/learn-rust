fn main() {
    let a0 = (1, true , "a");
    println!("a0 = {:?}", a0);
    println!("a0 = {:#?}", a0);

    let mut a1 = (1, true , "a");
    a1.1 = false;
    println!("a1 = {:?}", a1);

    let (z, _, zz) = a1;
    println!("z = {}\nzz = {}", z, zz);
}