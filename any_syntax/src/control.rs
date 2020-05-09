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
}