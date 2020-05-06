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

    let s = "こんにちは rust コードの世界";
    println!("s is {}", s);

    // 以下はバイト数で何文字目かを取りに行くので日本語だと、3バイト文字の中途半端なところを取りに行く
    // let hello = &s[0..5];
    // let world = &s[11..];

    // これならOK
    let hello = &s[0..15];
    let world = &s[21..];
    println!("こんにちは is {}", hello);
    println!("コードの世界 is {}", world);

    // 文字列をchar型のベクター型にして日本語を取り扱う方法
    let s = "This is ねこ😸neko 文字列";
    let mut v : Vec<char> = Vec::new();
    for char in s.chars() {
        v.push(char);
    }
    let v = &v[8..15];
    let mut s = String::new();
    for char in v {
        s.push(*char)
    }
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