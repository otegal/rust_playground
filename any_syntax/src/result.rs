use std::num::ParseIntError;

pub fn practice() {
    let r = "100".parse::<i32>();
    match r {
        Ok(n) => println!("n is {}", n),
        Err(e) => println!("error: {:?}", e),
    }

    // unwrapはResult型のOkの方だけを取得する
    let n = "100".parse::<i32>().unwrap();
    println!("n is {}", n);

    // let n: i32 = half_number("100");
    // println!("n is {}", n);

    // match half_number("100") {
    //     Ok(n) => println!("Ok is {}", n),
    //     Err(err) => println!("Error: {:?}", err),
    // }

    match half_number("100") {
        Ok(n) => println!("Ok: {}", n),
        Err(err) => println!("Error: {:?}", err)
    }
    match half_number("XXX") {
        Ok(n) => println!("Ok: {}", n),
        Err(err) => println!("Error: {:?}", err)
    }

    let n = "100".parse::<i32>().expect("これは数値ではありません");
    println!("n is {}", n);
    let n = "XXX".parse::<i32>().expect("これは数値ではありません");
    println!("n is {}", n);
}

fn half_number(s: &str) -> Result<i32, ParseIntError> {
    let n = s.parse::<i32>()?;
    Ok(n / 2)
}