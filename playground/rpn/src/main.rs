fn main() {
    // 6.1 + 5.2 * 4.3 - 3.4 / 2.5 * 1.6
    let exp = "6.1 5.2 4.3 * + 3.4 2.5 / 1.6 * -";
    let ans = rpn(exp);

    // デバッグビルドのときだけ答えが正しいかチェックする
    // 小数点以下4桁までを文字列に変換して比較する
    debug_assert_eq!("26.2840", format!("{:.4}", ans));

    println!("{} = {:.4}", exp, ans);
}

fn rpn(exp: &str) -> f64 {
    // mutでmutableな変数として利用することを宣言する
    // Vecは要素数が可変の配列的なデータ構造
    let mut stack = Vec::new();

    for token in exp.split_whitespace() {
        if let Ok(num) = token.parse::<f64>() {
            stack.push(num);
        } else {
            match token {
                // &は参照（ポインタ）を示す
                // |x, y| x + yはクロージャ。x, yを引数にとって、x + yを実行する
                "+" => apply2(&mut stack, |x, y| x + y),
                "-" => apply2(&mut stack, |x, y| x - y),
                "*" => apply2(&mut stack, |x, y| x * y),
                "/" => apply2(&mut stack, |x, y| x / y),

                _ => panic!("Unknown operator: {}", token),
            }
        }
    }

    // 行末に;をつけない事で処理の結果を関数の戻り値として返却できる。
    stack.pop().expect("Stack underflow")
}

fn apply2<F>(stack: &mut Vec<f64>, func: F)
where
    F: Fn(f64, f64) -> f64,
{
    if let (Some(y), Some(x)) = (stack.pop(), stack.pop()) {
        let z = func(x, y);
        stack.push(z);
    } else {
        panic!("Stack underflow");
    }
}