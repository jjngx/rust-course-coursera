// ===========================
// Rust: Ownership concept
// ===========================

fn main() {

    let x = 32.6;
    let y = x;

    println!("x: {}, y: {}", x, y);

    // borrowing
    let s1 = String::from("abc");
    let s2  = &s1;
    println!("s1: {}, s2: {}", s1, s2);

    // borrowing with Vectors
    let vec1 = vec![1,2,3,4,5,6];
    let vec2 = &vec1;
    println!("Vec1: {:?}, Vec2: {:?}", vec1, vec2);



    // really change ownership and make a clone
    let vec3 = vec1.clone();
    println!("Vec3: {:?}", vec3);

    // =========
    // Recap Rust ownership rules:
    // - each value in Rust has a variable that's called it's owner
    // - there can be only one owner at a time
    // - when the owner goes out of scope, the value is dropped
    // =========


    {
        let my_name = String::from("My Name");
        println!("{}", my_name);
    }
    // error as `my_name` is not in the scope and we do not have access to the scope above
    // println!("{}", my_name);


    // ==========
    // Ownership and reference in functions
    // ==========

    let stack_num = 32;
    let mut heap_vec = vec![1,2,3,4,5];

    stack_function(stack_num);
    heap_func(&heap_vec);

    // quiz

    let some_vec = vec![1,2,3,4];
    let ref1 = some_vec;
    let ref2 = &ref1;

    // =========
    //let mut vec_1 = vec![2,3,4,5];
    //let mut ref3 = &vec_1;


}

fn stack_function(mut stack_num: i32) {
    stack_num = 34;
    println!("Var: {}", stack_num);
}

fn heap_func(var: &Vec<i32>) {
    //var.push(50);
    println!("{:?}", var);
}
