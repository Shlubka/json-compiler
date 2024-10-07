fn main() -> i32 {
    let gg = 0;
    for i in 1..10 {
        println!("all ok");

        for i in 1..10 {
            println!("all ok");
        }

        loop {
            for i in 1..10 {
                println!("all ok");

                for i in 1..10 {
                    println!("all ok");
                }

                loop {
                    println!("loop");
                }

                while i < 10 {
                    println!("{i}");
                    i += 1;
                }
            }
        }

        while i < 10 {
            println!("{i}");
            i += 1;
        }
    }

    loop {
        println!("loop");
    }

    while i < 10 {
        println!("{i}");
        i += 1;
    }
    match gg {
        1 => println!("1"),
        2 => println!("2"),
        3 => println!("3"),
        4 => println!("4"),
        5 => println!("5"),
        5 => println!("5"),
        5 => println!("5"),
        5 => println!("5"),
        5 => println!("5"),
        5 => println!("5"),
        5 => println!("5"),
        5 => println!("5"),
        5 => println!("5"),
        5 => println!("5"),
    }
    return 1;
}
