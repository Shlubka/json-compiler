fn goyda() {
    if a > b {
        println!("a > b");
        println!("a > b");
        println!("a > b");
        println!("a > b");
        println!("a > b");
    }
    else {
        println!("a < b");
    }
}

fn main() -> i32 {
    goyda(3, 5);

    for i in 1..10 {
        println!("all ok");
    }
    return 1;
}
