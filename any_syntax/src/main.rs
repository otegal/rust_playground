fn main() {
    println!("Hello, world!");
    let sum = add(1, 2);
    println!("{}", sum);
}

fn add(x : i32, y : i32) -> i32 {
    println!("call add");
    x + y
}