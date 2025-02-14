fn main() {
    println!("Hello, world!");


    let mut some_vec = vec![2,3,4,5,6,5];

    println!("{:?}", some_vec);

    for i in some_vec.iter_mut(){
        *i +=5 ;
        println!("{:?}", i);
    }

    println!("{:?}", some_vec);
}
