#![allow(unused)]
fn main() {
    let arr: [i32; 5] = [5, 9, 8, 1, 2];
    let mut sum: i32 = 0;
    for e in arr {
        sum += e;
    }
    println!("Suma: {}", sum);
}  