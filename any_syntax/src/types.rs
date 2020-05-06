pub fn types_practice() {
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

    let an = add_two(10, 20);
    println!("an is {}", an);
    let an = add_one(30);
    println!("an is {}", an);

    let a = Sample::new(10);
    let ans = a.inc();
    println!("ans is {}", ans);
    let ans = a.add(20);
    println!("ans is {}", ans);

    let num = 10;
    let add_one = |x| {num + x};
    let add_two = |x, y| {x + y};

    let ans = add_one(1);
    println!("ans is {}", ans);
    let ans = add_two(10, 20);
    println!("ans is {}", ans);
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

fn add_two(x: i32, y: i32) -> i32 {
    x + y
}

fn add_one(x: i32) -> i32 {
    x + 1
}

struct Sample {
    x: i32,
}

impl Sample {
    fn new(x: i32) -> Sample {
        Sample {x: x}
    }
    fn inc(&self) -> i32 {
        self.x + 1
    }
    fn add(&self, x: i32) -> i32 {
        self.x + x
    }
}