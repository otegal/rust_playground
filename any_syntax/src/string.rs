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
}

fn test() {
    let ch = 'A';
    println!("ch is {}", ch);
    let u = ch as u8;
    println!("u is {}", u);
    let ch = u as char;
    println!("ch is {}", ch);
}