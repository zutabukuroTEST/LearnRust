fn main() {
    //mutable
    let mut s = String::from("hello world");

    //immutable参照(ここから不変なので可変にはできない)
    let word = first_word(&s);

    //OK
    println!("{}", s);

    //OK
    println!("{}", word);

    //NG(5行目で不変借用しており、18行目で使われているから可変借用できない)
    //s.clear();

    //OK
    println!("the first word is: {}", word);

    //OK
    s.clear();
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}