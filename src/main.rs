#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // メソッド記法
    fn area(&self) -> u32 {
        self.width * self.height
    }
    // メソッド記法
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // 関連関数
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main() {
    // お試し中
    // minus_one_calc(10);
    // let number = arithmetic_progression_calc(10);
    // println!("arithmetic progression result: {}", number);
    // let number = gaus_calc(10);
    // println!("gaus calc result: {}", number);

    // 以下学習中

    // // mutable
    // let mut s = String::from("hello world");

    // // immutable参照(ここから不変なので可変にはできない)
    // let word = first_word(&s);

    // // OK
    // println!("{}", s);

    // // OK
    // println!("{}", word);

    // // NG(5行目で不変借用しており、18行目で使われているから可変借用できない)
    // // s.clear();

    // // OK
    // println!("the first word is: {}", word);

    // // OK
    // s.clear();
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };
    let rect4 = Rectangle::square(10);
    println!("rect1 is {:#?}", rect1);
    println!("rect4 is {:?}", rect4);
    println!("The area of the rectangle is {} square pixels.",rect1.area());
    // rect1にrect2ははまり込む？
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect4));
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