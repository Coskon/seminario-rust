#![allow(unused)]
use std::io::stdin;

fn main() {
    let mut s: String = "Hola".to_string();
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Error leyendo la linea.");
    s += &input;  // o:   s.push_str(&input);
    //  s.make_ascii_uppercase();
    println!("{}", s.to_uppercase());
}