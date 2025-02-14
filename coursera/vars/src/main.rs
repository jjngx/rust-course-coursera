fn main() {
    println!("Hello, world!");


    let mut  x = 15.0;
    println!("{}", x);

    x = 20.3;

    println!("{}", x);


    // Data types
    // === Scalar ===

    // Integer
    // max i8
    println!("max number in i8 is: {}", std::i8::MAX);
    println!("min number of type i8 is {}", std::i8::MIN);


    // max u8
    println!("max number of type u8 is {}", std::u8::MAX);
    println!("min number of type u8 is {}", std::u8::MIN);


    // Float f32, f64

    let y = 4.2;
    let z:f32 = 34.1;
    // Bool

    println!("some our vars are: {:?}", (x, y, z));

    // Char

    let c1 = 'a';
    let c2 = '3';
    let c3 = '+';
    let c4 = '\u{288A}';
    let c5 = '\"';

    println!("values are c1 = {}, c2 = {}, c3 = {}, c4 = {}, c5 = {}", c1, c2, c3, c4, c5);


    // let overflow_number:u8 = 100;

    // representations
    let xy:i32 = 30;
    println!("the value in octal: {:o}, hex {:X}, binary {:b}", xy, xy, xy);

    let number = 45;

    let n1 = 14;
    let n2 = 23.7;
    let n3 = n1 as f64 + n2;

    println!("{}, {}, {}, {}", number, n1, n2, n3);

    // Shadowing

    let c1 = 20;
    let c1 = 34 + 23;
    println!("{}", c1);

    // shadowing by immutable variable

    //let mut b1 = 20;
    let b1 = 20 + 12;
    println!("shadowed {}", b1);


    // shadowing in a scope
    let mut d1 = 10;
    {
        let d1 = 20;
        println!("inside the scope d = {}", d1);
    }
    println!("outside the scope d = {}", d1);


    // shadowing - in closure example
    let mut d2 = 4;
    println!("{}", d2);
    {
        d2 = 5;
        println!("{}", d2);
    }
    println!("{}", d2);


    // Constants

    const MAX_SPEED:u8 = 120;
    println!("max speed is {}", MAX_SPEED);

}
