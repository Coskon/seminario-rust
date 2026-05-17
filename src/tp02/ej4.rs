#![allow(unused)]
use super::ej1::es_par;

pub fn cantidad_impares(arr: &[i32]) -> i32 {
    let mut cant = 0;
    for n in arr {
        if !es_par(*n) {
            cant += 1;
        }
    }
    return cant;
}

#[test]
fn test_cantidad_impares() {
    assert_eq!(cantidad_impares(&[1, 2, 3, 4]), 2);
    assert_eq!(cantidad_impares(&[]), 0);
    assert_eq!(cantidad_impares(&[-3, -1, 1, 3]), 4);
    assert_eq!(cantidad_impares(&[-6, -4, -2, 0, 2, 4, 6]), 0);
}