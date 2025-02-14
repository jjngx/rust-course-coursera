use std::ops::Index;

fn main() {
    println!("Compond data types!");

    // Compound datatypes
    // tuple, arrays

    // tuple

    let salary = ("Salary", 40_000);
    println!("{}, {}", salary.0, salary.1);

    // print tuple at once
    println!("{:?}", salary);

    let (name, value) = salary;
    println!("{}, {}", name, value);

    let salary_name = name;
    let salary_value = value;
    println!("{}, {}", salary_name, salary_value);
    
    // nested tuples

    let nested_tpl = (2, 5.6, (2, 4.5), "Hello!");
    println!("{:?}", nested_tpl);

    let msg = nested_tpl.3;
    println!("{}", msg);

    // === arrays ===

    let mut my_array: [u8;3] = [0,1,2];
    println!("{:?}", my_array);

    my_array[0] = 10;
    println!("{:?}", my_array);


    println!("{:?}", my_array[1]);

    // initialise array with x number of elements
    let my_arr = [0; 10];
    println!("{:?}", my_arr);

    // array of strings

    let mut string_array = ["dupa", "blada"];
    println!("{:?}", string_array);
    let str2 = ["Bolek"; 3];
    println!("{:?}", str2);

    // slicing arrays
    let mut nums = [1,2,3,4,5];
    println!("{:?}", nums);
    let sub = &nums[1..3];
    println!("{:?}", sub);
    println!("{:?}", sub);
    println!("{:?}", nums);
    
    println!("number of elements in the array: {}", nums.len());
    println!("size the array occupies: {}", std::mem::size_of_val(&nums));

    // checking index
    let check_index = nums.get(100);
    println!("{:?}", check_index);




    // ===============
    // === Vectors ===
    // ===============

    let mut v1 = vec![1,2,3,4,5,6,7,8,9,10];
    println!("{}", v1[2]);

    let mut v2 = vec![0;10];
    println!("{:?}", v2);
    v2[0] = 20;
    println!("{:?}", v2);

    let mut v3 = vec!['a', 'b', 'c', 'd', 'e', 'f'];
    let v4: &&[i32] = &&v1[0..3];
    println!("{:?}", v4);

    // add and remove elements
    // check index

    let ci = v3.get(1);
    println!("{:?}", ci);

    v3.push('z');
    println!("{:?}", v3);

    v3.remove(2);
    println!("{:?}", v3);





}
