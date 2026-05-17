#![allow(unused)]
use super::ej1::es_par;

pub fn suma_pares(arr: &[i32]) -> i32 {
    let mut sum = 0;
    for n in arr {
        if es_par(*n) {
            sum += *n;
        }
    }
    return sum;
}

#[test]
fn test_suma_pares() {
    assert_eq!(suma_pares(&[1, 2, 4, 3, 6, 99]), 12);
    assert_eq!(suma_pares(&[1, 3, 99]), 0);
    assert_eq!(suma_pares(&[]), 0);
    assert_eq!(suma_pares(&[-4, 4, -2]), -2);
}