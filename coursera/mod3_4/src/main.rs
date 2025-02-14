fn main() {
    println!("Hello, world!");


    let mut heap_num = vec![1,2,3];
    // let ref1 = &mut heap_num;
    // let ref2 = &mut heap_num;
    
    
    let ref1 = &heap_num;
    let ref2 = &heap_num;

    
    println!("ref1: {:?}, ref2: {:?}", ref1, ref2);

    let out = add(3,3);
    println!("{out}");

}


fn add(a: i32, b: i32) -> i32 {
    a + b
}
