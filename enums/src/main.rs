enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Coin {
    Penny,
    Quarter,
    Nickle,
}

fn main() {
    // Using enum variants
    let _home = IpAddr::V4(127, 0, 0, 1);

    let _loopback = IpAddr::V6(String::from("::1"));

    // Extracting the value out of a Some type
    let mut some_1 = Some(8);

    let x = some_1.take();

    let mut some_2 = Some(1);

    let z = some_2.get_or_insert(1);

    let some_3 = Some(String::from("Hello"));

    let string = some_3.unwrap();

    println!("some_1: {:#?}", some_1);
    println!("x: {:?}", x);
    println!("z: {:?}", z);
    println!("string: {}", string);

    // Using pattern matching with enums and the Option type
    let penny = Coin::Penny;
    let quarter = Coin::Quarter;
    let nickle = Coin::Nickle;

    let number: Option<u8> = Some(1);

    println!(
        "x: {:#?}",
        match number {
            Some(val) => val,
            None => 0,
        }
    );

    println!("penny: {}", get_coin_value(penny));
    println!("quarter: {}", get_coin_value(quarter));
    println!("nickle: {}", get_coin_value(nickle));
}

fn get_coin_value(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Quarter => 25,
        Coin::Nickle => 5,
    }
}
