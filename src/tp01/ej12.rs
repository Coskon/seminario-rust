#![allow(unused)]
fn main() {
    let tupla: (&str, [i32; 7]) = ("hola mundo", [1, 2, 3, 4, 5, 6, 7]);
    println!("Cadena: {}, Suma: {}", tupla.0, {let mut sum: i32 = 0; for e in tupla.1 { sum += e; } sum });
}

/* Version mas legible:
fn main() {
    let tupla: (&str, [i32; 7]) = ("hola mundo", [1, 2, 3, 4, 5, 6, 7]);
    let mut sum: i32 = 0; 
    for e in tupla.1 { 
        sum += e; 
    } 
    return sum;
    println!("Cadena: {}, Suma: {}", tupla.0, sum);
}
*/