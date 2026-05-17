#![allow(unused)]
use std::io::stdin;

fn main() {
    let bool_val: bool = true;
    let mut input = String::new();
    print!("Ingresar valor booleano: "); stdin().read_line(&mut input).expect("Error leyendo la linea.");
    let bool_input: bool = input.trim().parse::<bool>().expect("Valor de entrada no es un booleano.");

    println!("{} && {} = {}", bool_val, bool_input, bool_val && bool_input);
    println!("{} || {} = {}", bool_val, bool_input, bool_val || bool_input);
}