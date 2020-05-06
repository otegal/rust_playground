fn main() {
    let dog = "dog";
    let cat = "cat";
    println!("{} and {}", dog, cat);

    let s1 = String::from("Hello");
    let s2 = String::from("Rust");
    let s3 = String::from("World");
    let s = s1 + " " + &s2 + " " + &s3;
    println!("{}", s)
}
