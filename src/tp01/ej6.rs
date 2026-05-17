#![allow(unused)]
use std::io::stdin;

fn main() {
    let num: u32 = 4832;
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Error leyendo la linea.");
    let num_input = input.trim().parse::<u32>().expect("Numero ingresado no es un entero sin signo.");
    println!("({}+{})^2 = {}", num, num_input, (num+num_input).pow(2));
}