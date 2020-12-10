//Welcome to AtCoder
/*
高橋君はデータの加工が行いたいです。
整数 a,b,ca,b,cと、文字列 ss が与えられます。 a+b+ca+b+c の計算結果と、文字列 ss を並べて表示しなさい。
*/

use std::io;

fn main() {
    // 一つ目
    
    let mut s1 = String::new();// 新しいバッファを確保
    // read_lineの文字列は可変じゃなきゃ✖ 戻り値：Result型
    io::stdin().read_line(&mut s1).expect("input number");// エラーハンドリング
    // trimで両端の空白\nを削除
    // parse キャスト　Result型を返す
    // ok は Result<T, E> を Option<T> に変換する
    // Result.unwrap メソッドは Ok なら中の値を返し、Err なら panic を起こす。
    // Option も unwrap メソッドを持っており、Some なら中の値を返し、Noneならpanic
    let num1: i32 = s1.trim().parse().ok().unwrap();
    //println!("{}",num1);

    // 二つ目
    let mut s2 = String::new();
    io::stdin().read_line(&mut s2).expect("input number");
    let  (n, m) = {
        let mut ws = s2.split_whitespace(); // 空白区切りの単語に分解する
        // next(): Iterator<T> -> Option<T>
        // 1 Iterator<T> -> Option<T>
        // 2 Option<T> -> T
        // 3 T -> i32 にキャスト(parse) 戻り値Result
        // 4 Result<T,E>
        let n: i32 = ws.next().unwrap().parse().unwrap(); // イテレータから値を取り出して整数に
        let m: i32 = ws.next().unwrap().parse().unwrap();
        (n, m)};
    let num2 = (n,m);
    //println!("{} {}",num2.0,num2.1);

    // 三つ目
    let mut s3 = String::new();
    io::stdin().read_line(&mut s3).expect("input string");
    let s = s3.trim();

println!("{} {}",num1+num2.0+num2.1,s);
   
}
