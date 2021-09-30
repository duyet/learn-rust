fn _if() {
    let a = 5;

    if a < 5 {
        println!("a < 5");
    } else if a < 10 {
        println!("a < 10")
    } else {
        println!("a > 10")
    }

    let age: i8 = 18;
    let below_18 = if age < 18 { true } else { false };
    println!("{}", below_18);
}

fn _match() {
    let a = 10;
    let size = match a {
        16 => "S",
        17 | 18 => "M", // 17 or 18
        19..=21 => "L", // (19, 20, 21)
        22 => "XL",
        _ => "NA",
    };
    println!("{}", size);
}

fn _match_2() {
    let is_allowed = false;
    let res = match is_allowed {
        true => "ok",
        false => "nok",
    };
    println!("{}", res);
}

fn _match_3() {
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
}

fn _loop() {
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
}

fn _vectors() {
    let mut a = Vec::new();
    let mut b = vec![];
    let mut c = vec![1, 2, 3];
    let mut d: Vec<i8> = Vec::new();
    let mut e: Vec<i8> = vec![1i8, 2, 3];
    let mut f = vec![1; 10]; // 1,1,1,1,1,1....
}

fn main() {
    _match();
    _match_2();
    _match_3();
    _loop();
}
