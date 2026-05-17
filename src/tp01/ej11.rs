#![allow(unused)]
use std::io::stdin;

fn main() {
    let arr: [&str; 5] = ["hola", "mundo", "que", "tal", "estas"];
    let mut input: String = String::new();
    stdin().read_line(&mut input).expect("Error leyendo la linea.");
    let res: &str = input.trim();
    let mut esta: bool = false;
    for i in 0..5 {
        if res == arr[i] {
            esta = true;
            break;
        }
    }
    if esta {
        println!("La cadena se encuentra en el arreglo.");
    } else {
        println!("La cadena no se encuentra en el arreglo.");
    }
}