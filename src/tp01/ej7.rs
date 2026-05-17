#![allow(unused)]
fn main() {
    let mut arr: [i32; 6] = [1, 2, 4, 8, 16, 32];
    const VALOR: i32 = 5;
    for i in 0..6 {
        arr[i] = arr[i]*VALOR;
    }
    println!("Constante: {}, Arreglo modificado: {:?}", VALOR, arr);
}