fn main() {
    let s = String::from("Hello World");

    let word = first_word(&s);
}

first_word(x: String) -> usize {
    let bytes = x.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
