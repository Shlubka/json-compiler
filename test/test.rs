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
        if (condition1) {
            statement1;
            if (condition2) {
                statement2;
                if (condition3) {
                    statement3;
                } else if (condition4) {
                    statement4;
                } else {
                    statement5;
                }
            } else if (condition6) {
                statement6;
            } else {
                statement7;
            }
        } else if (condition8) {
            statement8;
        } else {
            statement9;
        }
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
