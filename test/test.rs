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
            println!("statement6");
        } else {
            println!("statement7");
        }
    } else if condition8 {
        println!("statement8");
    } else {
        println!("statement9");
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
        println!("{}", gg);
        break; // Добавлено для предотвращения бесконечного цикла
    }

    match gg {
        1 => println!("1"),
        2 => println!("2"),
        3 => println!("3"),
        4 => println!("4"),
        5 => println!("5"),
        _ => println!("other"),
    }

    return 1;
}
