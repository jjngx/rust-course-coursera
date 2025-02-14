// ==========
// Functions!
// ==========

use core::str;
use std::io::Stdin;

fn main() {
    hello("Bolek", 156);

    let n1 = 23;
    let n2 = 24;

    println!("{}", mysum(n1, n2));


    println!("{:?}", calculate(2, 5));
    let out = calculate(10,20);
    println!("{}, {}", out.0, out.1);


    let fullname = {
        let f1 = out.0;
        let f2 = out.1;
        format!("{} =>>> {}", f1, f2)
    };
    println!("{}", fullname);



    // get input from terminal
    let mut n :String = String::new();
    std::io:: stdin()
    .read_line(&mut n)
    .expect("failed to read input");

    let n3: f64 = n.trim().parse().expect("error!");
    println!("{:?}", n3)

}

fn hello(name: &str, high: u8 ) {
    println!("Your name is {}, your high is {}", name, high);
}

fn mysum(number1: i32, number2: i32) -> i64 {
    let out = number1 + number2;
    out as i64
}

fn calculate(n1: i32, n2: i32) -> (i32, i32) {
    (n1 - n2, n1 + n2)
}



