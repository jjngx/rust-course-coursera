//use std::io::stdin;

fn main() {
    // println!("Hello, world!");



    // let value: i32 = if true {
    //     1
    // } else {
    //     2
    // };

    // println!("{value}");

    // match statement

    // let num = 3;
    // match num {
    //     1 => println!("1"),
    //     2 | 3 => println!("2 or 3"),
    //     4..=100 => println!("4 -> 100"),
    //     101.. => println!("uuuuu"),
    //     _ => println!("default")
    // }


    // let marks = 98;
    // let mut grade: char = match marks {
    //     90..=100 => 'A',
    //     _ => 'F',
    // };
    // println!("{grade}")



    // Game

    let my_number = 5;
    let mut guess = false;

    println!("Guess my number please!");

    while guess != true {
        let mut number: String = String::new();
        std::io::stdin().read_line(&mut number).expect("failed to read number");

        let number: u8 = number.trim().parse().expect("invalid input");

        if number == my_number {
            println!("You guessed!");
            guess = true;
        } else {
            println!("try again");
        }
    }
}
