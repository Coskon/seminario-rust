#![allow(unused)]
use super::ej1::es_par;

pub fn reemplazar_pares<const N: usize>(arr: &mut [i32; N]) {
    for n in arr {
        if es_par(*n) {
            *n = -1;
        }
    }
}

#[test]
fn test_reemplazar_pares() {
    let mut arr1 = [1, 2, 3, 4];
    reemplazar_pares(&mut arr1);
    assert_eq!(arr1, [1, -1, 3, -1]);

    let mut arr2 = [];
    reemplazar_pares(&mut arr2);
    assert_eq!(arr2, []);

    let mut arr3 = [-8, 0, 0];
    reemplazar_pares(&mut arr3);
    assert_eq!(arr3, [-1, -1, -1]);

    let mut arr4 = [-5, -4, -3, -2, -1, 0, 1, 2, 3];
    reemplazar_pares(&mut arr4);
    assert_eq!(arr4, [-5, -1, -3, -1, -1, -1, 1, -1, 3]);
}