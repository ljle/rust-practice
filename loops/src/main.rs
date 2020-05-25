fn main() {
    // while_with_loop()
    // while_loop()
    for_loop()
}

fn for_loop () {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
}

fn while_loop() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("Loop ended!");
}

fn while_with_loop() {
   let mut counter: u32 = 0;

   loop {
       if counter <= 10 {
           println!("Counter is at {}", counter);
           counter += 1;
       } else {
           break
       }
   }
}