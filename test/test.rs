/*fn match_test() {
    while true {
        match gg {
            1 => println!("1"),
            2 => {
                println!("2");
                println!("2");
            }
            3 => {
                println!("3");
                println!("3");
                println!("3");
            }
            3 => {
                println!("3");
                println!("3");
                println!("3");
            }
            3 => {
                println!("3");
                println!("3");
                println!("3");
            }
            3 => {
                println!("3");
                println!("3");
                println!("3");
            }
            3 => {
                println!("3");
                println!("3");
                println!("3");
            }
            3 => {
                println!("3");
                println!("3");
                println!("3");
            }
            3 => {
                println!("3");
                println!("3");
                println!("3");
            }
            3 => {
                println!("3");
                println!("3");
                println!("3");
            }
            3 => {
                println!("3");
                println!("3");
                println!("3");
            }
            3 => {
                println!("3");
                println!("3");
                println!("3");
            }
            3 => {
                println!("3");
                println!("3");
                println!("3");
            }
            3 => {
                println!("3");
                println!("3");
                println!("3");
            }
            3 => {
                println!("3");
                println!("3");
                println!("3");
            }
            4 => {
                println!("4");
                println!("4");
                println!("4");
                println!("4");
            }
            _ => println!("other"),
        }
    }
    println!("end");
}*/

fn decision_tree(value: i32) {
    if value < 2 {
        println!("Значение меньше 2.");
    } else if value < 5 {
        println!("Значение от 2 до 5.");
    } else if value < 10 {
        println!("Значение от 5 до 10.");
    } else if value < 15 {
        println!("Значение от 10 до 15.");
    } else if value < 20 {
        println!("Значение от 15 до 20.");
    } else if value < 30 {
        println!("Значение от 20 до 30.");
    } else {
        println!("Значение 30 или больше.");
    }
}
/*
fn new_test() {
    if g11 {
        println!("g11")
    } else {
        println!("g2")
    }
}*/
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
        }
    } else if condition8 {
        println!("statement4");
    } else {
        println!("statement5");
    }

    return true;
}
*/
/*fn main() -> i32 {
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
