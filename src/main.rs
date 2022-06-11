fn main() {
    // お試し中
    minus_one_calc(10);
    let number = arithmetic_progression_calc(10);
    println!("arithmetic progression result: {}", number);
    let number = gaus_calc(10);
    println!("gaus calc result: {}", number);

    // 以下学習中

    // mutable
    let mut s = String::from("hello world");

    // immutable参照(ここから不変なので可変にはできない)
    let word = first_word(&s);

    // OK
    println!("{}", s);

    // OK
    println!("{}", word);

    // NG(5行目で不変借用しており、18行目で使われているから可変借用できない)
    // s.clear();

    // OK
    println!("the first word is: {}", word);

    // OK
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

//減算再帰
fn minus_one_calc(num: u32) -> u32 {
    println!("minus one number: {}", num);
    let num = num - 1;
    if num == 0 { num } else { minus_one_calc(num) }
}

//等差数列再帰
fn arithmetic_progression_calc(num: u64) -> u64 {
    match num {
        0 => 0,
        1 => 1,
        _ => num + arithmetic_progression_calc(num - 1)
    }
}

// ガウス計算
fn gaus_calc(num: u32) -> u32 {
    ((1 + num)*num)/2
}