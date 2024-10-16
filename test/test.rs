fn match_test() {
    match gg {
        1 => println!("1"),
        2 => println!("2"),
        3 => println!("3"),
        4 => println!("4"),
        5 => println!("5"),
        _ => println!("other"),
    }
    println!("end");
}

/*
fn goyda() -> bool {
    let condition1 = true;
    let condition2 = true;
    let condition3 = true;
    let condition4 = false;
    let condition6 = false;
    let condition8 = false;

    if condition1 {
        println!("statement1");
        if condition2 {
            println!("statement2");
        } else {
            println!("statement3");
        }
    } else if condition8 {
        println!("statement4");
    } else {
        println!("statement5");
    }

    return true;
}

fn main() -> i32 {
    let gg = 0;

    for i in 1..10 {
        println!("all ok");
    }

    loop {
        goyda();
        break; // Добавлено для предотвращения бесконечного цикла
    }

    while gg < 10 {
        if condition1 {
            println!("statement1");
            if condition2 {
                println!("statement6");
            } else {
                println!("statement7");
            }
        } else if condition8 {
            println!("statement8");
        } else {
            println!("statement9");
        }
        break; // Добавлено для предотвращения бесконечного цикла
    }


    return 1;
}
*/
