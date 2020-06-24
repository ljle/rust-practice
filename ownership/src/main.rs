fn main() {
    let mut s = String::from("hello");

    change(&mut s);

    println!("{}", s);

    let name = String::from("John Doe");

    print!("{}", first_word(&name));

    print!("{}", first_word("Johnny"));

    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    print!("{:?}", slice);
}

fn change(string: &mut String) {
    string.push_str(", world");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
