//Welcome to AtCoder

use std::io;

fn main() {
    // 一つ目
    let mut s1 = String::new();
    io::stdin().read_line(&mut s1).expect("input number");
    let num1: i32 = s1.trim().parse().ok().unwrap();
    //println!("{}",num1);

    // 二つ目
    let mut s2 = String::new();
    io::stdin().read_line(&mut s2).expect("input number");
    let  (n, m) = {
        let mut ws = s2.split_whitespace(); // 空白区切りの単語に分解する
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
