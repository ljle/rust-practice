fn main() {
    // Unused variable
    let _a: u32 = 8;

    // Mutable variable
    let mut b: u32 = 7;
    b = b + 1;

    println!("{}!", b);

    // Array
    let arr1 = [1, 2, 3];

    println!("arr1: {:?}", arr1);

    // Array with length 5 and elements value of 3
    let arr2 = [3; 5];

    println!("arr2: {:?}", arr2);

    // Signed integer
    let z: i32 = -1;
    
    println!("z: {}", z);
}
