#![allow(unused)]
use std::io::stdin;

fn main() {
    let num: f32 = 5.0;

    let mut input = String::new();
    print!("Ingresar valor flotante: "); stdin().read_line(&mut input).expect("Error leyendo la linea.");
    let input_f32: f32 = input.trim().parse::<f32>().expect("Valor de entrada no es un numero."); 

    println!("{} + {} = {}", num, input_f32, num+input_f32);
    println!("{} - {} = {}", num, input_f32, num-input_f32);
    println!("{} * {} = {}", num, input_f32, num*input_f32);
    println!("{} / {} = {}", num, input_f32, num/input_f32);
}