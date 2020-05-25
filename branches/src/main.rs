fn main() {
    let condition = true;

    // Error for incompatible types
    let number = if condition { 5 } else { "six" };

    println!("The value of number is: {}", number);
}