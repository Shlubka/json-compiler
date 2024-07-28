fn goyda () {
    if a > b {
        printlm!("a > b");
        printlm!("a > b");
        printlm!("a > b");
        printlm!("a > b");
        printlm!("a > b");
    }
    else {
        printlm!("a < b");
    }
}

fn main () -> i32 {
    goyda(3, 5);

    printlm!("all ok");
    return 1;
}
