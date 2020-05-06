fn main() {
    let dog = "dog";
    let cat = "cat";
    println!("{} and {}", dog, cat);

    let s1 = String::from("Hello");
    let s2 = String::from("Rust");
    let s3 = String::from("World");
    let s = s1 + " " + &s2 + " " + &s3;
    println!("{}", s);

    let t = ("hh", 30);
    println!("name is {} age {}", t.0, t.1);

    let a = ["A", "B", "C"];
    println!("{}", a[0]);
    println!("{}", a[2]);


    let z = String::from("Hello");
    let len = string_length(&z);
    println!("len is {}", len);
    println!("z is {}", z);

    let mut q = 100;
    println!("q is {}", q);
    q = 200;
    println!("q is {}", q);

    let w = 100;
    println!("w is {}", w);
    {
        let w = 200;
        println!("w is {}", w);
    }
    println!("w is {}", w);

    let mut answer = test(-9);
    println!("answer is {}", answer);

    answer = test(200);
    println!("answer is {}", answer);

    answer = test(50);
    println!("answer is {}", answer);

}

fn string_length(s : &String) -> usize {
    let length = s.len();
    length
}

fn test(x: i32) -> i32 {
    let mut ans = x;
    if x < 0 {
        ans = 0;
    }
    if x > 100 {
        ans = 100;
    }
    ans
}
