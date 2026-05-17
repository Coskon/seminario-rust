#![allow(unused)]
use std::io::stdin;

fn count_char(s: &str, c: char) -> u32 {
    let mut count: u32 = 0;
    for ch in s.chars() {
        if ch == c {
            count += 1;
        }
    }
    return count;
}

fn main() {
    const STRING: &str = "Hola Mundo";
    let mut input: String = String::new();
    stdin().read_line(&mut input).expect("Error leyendo el caracter.");
    let c: char = input.chars().next().expect("No es un caracter.");
    println!("{}", count_char(STRING, c));
}