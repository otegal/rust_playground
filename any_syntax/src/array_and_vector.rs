pub fn practice() {
    let ary = [1, 2, 3, 4, 5];
    println!("ary[0] is {}", ary[0]);
    println!("ary[4] is {}", ary[4]);
    println!("ary.len is {}", ary.len());

    let ary: [u8; 5] = [0x10, 0x20, 0x30, 0x40, 0x50];
    println!("ary[0] is {}", ary[0]);
    println!("ary[4] is {}", ary[4]);
    println!("ary.len is {}", ary.len());

    let mut ary: [u8; 16] = [0; 16];
    println!("ary[0] is {}", ary[0]);
    println!("ary[15] is {}", ary[15]);
    ary[0] = 10;
    println!("ary[0] is {}", ary[0]);
    println!("ary[15] is {}", ary[15]);

    let a = [1u8, 2u8, 3u8, 4u8];
    unsafe {
        let b = std::mem::transmute::<[u8; 4], u32>(a);
        println!("b is {:X}", b);
    }
    let a: u32 = 0x11223344;
    unsafe {
        let b = std::mem::transmute::<u32, [u8; 4]>(a);
        println!("b[] is {:X} {:X} {:X} {:X}", b[0], b[1], b[2], b[3])
    }

    let v = vec![1, 2, 3, 4, 5];
    println!("v[0] is {}", v[0]);
    println!("v[4] is {}", v[4]);
    println!("v.len is {}", v.len());

    println!("v.get(0) is {:?}", v.get(0));
    println!("v.get(4) is {:?}", v.get(4));
    println!("v.get(0) is {}", v.get(0).unwrap());
    println!("v.get(4) is {}", v.get(4).unwrap());

    let mut v: Vec<i32> = Vec::new();
    println!("v.len is {}", v.len());
    v.push(10);
    v.push(20);
    v.push(30);
    println!("v.len is {}", v.len());
    println!("pop is {:?}", v.pop());
    println!("pop is {:?}", v.pop());
    println!("pop is {:?}", v.pop());
    println!("v.len is {}", v.len());

    let v = vec![1, 2, 3, 4, 5];
    print!("for is ");
    for i in &v {
        print!("{} ", i);
    }
    println!("");

    print!("for and iter is ");
    for i in v.iter() {
        print!("{} ", i);
    }
    println!("");

    let v = vec![1, 2, 3, 4, 5];
    let lst = v.iter().map(|x| x * 10);
    for i in lst {
        println!("i is {}", i);
    }
}