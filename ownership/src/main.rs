fn main() {
    let mut s = String::from("hello");

    change(&mut s);

    println!("{}", s);

    let name_s = String::from("John Doe");

    print!("{}", first_word(&name_s));
}

fn change(string: &mut String) {
    string.push_str(", world");
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
