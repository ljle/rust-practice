#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String,
}

#[derive(Debug)]
struct Point {
    x: u64,
    y: u64
}

fn main() {
    let person1 = Person {
        first_name: String::from("John"),
        last_name: String::from("Doe"),
    };

    let _person2 = Person {
        first_name: String::from("Jane"),
        ..person1
    };

    let x = 1;
    let y = 2;

    let p = Point { x, y };

    println!("{:#?}", p);
}
