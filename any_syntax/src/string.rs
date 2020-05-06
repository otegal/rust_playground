pub fn string_practice() {
    let ch = 'A';
    println!("ch is {}", ch);
    let ch = 'あ';
    println!("ch is {}", ch);
    let ch = '😃';
    println!("ch is {}", ch);
    let ch = '\u{1F431}';
    println!("ch is {}", ch);

    test();

    let s = "hello rust world.";
    println!("s is {}", s);

    let hello = &s[0..5];
    let world = &s[11..];
    println!("{} {}", hello, world);

    let len = s.len();
    println!("len is {}", len);

    let mut s = String::new();
    s.push_str("hello ");
    s.push_str("rust ");
    s.push_str("world.");
    println!("s is {}", s);

    let hello = "HELLO ";
    let rust = "RUST ";
    let world = "WORLD.";
    let s = format!("{}{}{}", hello, rust, world);
    println!("s is {}", s);

    // &str型 　 後から変更出来ない文字列
    // &String型 変更出来る文字列
    let s = "hello rust world.".to_string();
    println!("s is {}", s);
    let s = String::from("hello rust world.");
    println!("s is {}", s);
}

fn test() {
    let ch = 'A';
    println!("ch is {}", ch);
    let u = ch as u8;
    println!("u is {}", u);
    let ch = u as char;
    println!("ch is {}", ch);
}