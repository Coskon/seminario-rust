#![allow(unused)]
fn main() {
    let arr1: [i32; 5] = [1, 2, 3, 4, 5];
    let arr2: [i32; 5] = [6, 7, 8, 9, 10];
    let mut arr3: [i32; 5] = [0; 5]; // crea array de 0s de tamaño 5

    for i in 0..5 {
        arr3[i] = arr1[i] + arr2[i];
    }

    println!("{:?}", arr3);
}