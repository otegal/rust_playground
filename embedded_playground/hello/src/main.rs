static mut X: i32 = 30;

fn main() {
    println!("Hello, world!");

    println!("Hello, {}!", "world"); // {}を使う場合、std::fmt::Displayトレイトを実装している場合のみ。

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("a = {:?}", a);  //  {:?}を使うと表示できる場合もある
    // println!("a = {}", a); // こっちだとコンパイルエラーになる


    unsafe {
        X += 1; // ←これを外に出すとコンパイルエラー。わかりやすいエラー文出てきまっせ
        println!("X = {}", X);
    }
}
