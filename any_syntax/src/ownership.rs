#[derive(Debug)]
struct Person {
    name: &'static str,
    age: i32,
}

fn print_a(a: &Person) {
    println!("print_a: a is {:?}", a);
}

fn add_age(a: &mut Person) {
    a.age += 1;
}

pub fn practice() {
    let a = Person { name: "hh", age: 30 };
    print_a(&a);
    println!("main: a is {:?}", a);

    let mut a = Person { name: "hh", age: 30 };
    println!("a is {:?}", a);
    add_age(&mut a);
    println!("a is {:?}", a);

    let a = 100;
    println!("a is {}", a);
    let x = a;
    println!("x is {}", x);
    let y = a;
    println!("y is {}", y);
}