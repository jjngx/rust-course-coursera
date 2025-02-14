use std::io::Stdin;




fn new_stack(maxsize: usize) -> Vec<u32> {
    let vec: Vec<u32> = Vec::with_capacity(maxsize);
    vec
}

fn pop(stack: &mut Vec<u32>) -> Option<u32> {
    let poped_val: Option<u32> = stack.pop();
    println!("poped value is: {:?}", poped_val);
    poped_val
}

fn push(stack: &mut Vec<u32>, item: u32, maxsize: usize) {
    if stack.len() == maxsize {
        println!("can't add more elements")
    } else {
        stack.push(item);
        println!("{:?}", stack)
    }
}

fn size(stack: &Vec<u32>) -> usize {
    stack.len()
}

fn input() -> u32 {
    let mut n: String = String::new();
    std::io::stdin()
    .read_line(&mut n)
    .expect("failed to read number");

    let n: u32 = n.trim().parse().expect("invalid input");
    n
}

fn main() {
    println!("What stack size do you want?");
    let size_stack = input();
    let mut stack: Vec<u32> = new_stack(size_stack as usize);

    println!("\n\n *** Menu *** \n");
    println!("1. Push\n2. Pop\n3. Display\n4.Size\n5. Exit");
    println!("Enter your choice");
    let choice = input();

    match choice {
        1 => {
            println!("Enter number to push to the stack");
            let item = input();
            push(&mut stack, item,size_stack as usize);
        },
        2 => println!("The element is {:?}", pop(&mut stack)),
    }

}
