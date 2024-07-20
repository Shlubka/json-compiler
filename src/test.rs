    fn extun() -> i32 {
    let a:i32 = 12;
    let b:i32 = 21;
    if a > b {
        a - b
    }
    else {
        return b - a
    }
}

fn main () {
    println!("hi from test.rs");

    let c = extun();
}

