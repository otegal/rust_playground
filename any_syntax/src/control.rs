#[derive(Debug)]
enum LANG {
    JAPANESE = 81,
    ENGLISH = 44,
    CHINESE = 86,
    FRANCH = 33,
}

enum Option<T> {
    Some(T),
    None,
}

pub fn control_practice() {
    let v = vec![10, 20, 30, 40, 50];
    print!("v is ");
    for i in &v {
        print!("{} ", i);
    }
    println!("");

    // イテレータ
    for i in v.iter() {
        print!("{} ", i);
    }
    println!("");

    // indexあり
    for (index, value) in v.iter().enumerate() {
        print!("{}:{} ", index, value);
    }
    println!("");

    let lang = LANG::JAPANESE;
    println!("lang is {}", lang as i32);

    let lang = LANG::JAPANESE;
    let m = match lang {
        LANG::JAPANESE => "日本語",
        LANG::ENGLISH => "英語",
        LANG::CHINESE => "中国語",
        LANG::FRANCH => "フランス語",
        _ => "未定義"
    };
    println!("lang is {}", m);

    let x = Some(10);
    let v = match x {
        Some(i) => i,
        None => -1,
    };
    println!("v is {}", v);

    let x = Some(10);
    match x {
        Some(i) => println!("i is {}", i),
        _ => (),
    };
    if let Some(i) = x {
        println!("i is {}", i);
    }

    let x = 3;
    let m = match x {
        1 => "one",
        2 => "two",
        3 => "three",
        _ => "other",
    };
    println!("m is {}", m);
}