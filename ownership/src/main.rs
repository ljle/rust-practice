fn main() {
    let mut s = String::from("hello");

    change(&mut s);

    print!("{}", s);
}

fn change(string: &mut String) {
    string.push_str(", world");
}
