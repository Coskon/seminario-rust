#![allow(unused)]
pub fn cantidad_de_mayores(arr: &[i32], limite: i32) -> i32 {
    let mut cant = 0;
    for n in arr {
        if *n > limite {
            cant += 1;
        }
    }
    cant
}

#[test]
fn test_cantidad_de_mayores() {
    assert_eq!(cantidad_de_mayores(&[1, 2, 3, 4, 5, 6], 3), 3);
    assert_eq!(cantidad_de_mayores(&[], 3), 0);
    assert_eq!(cantidad_de_mayores(&[], 0), 0);
    assert_eq!(cantidad_de_mayores(&[-10, -20, -30], -20), 1);
    assert_eq!(cantidad_de_mayores(&[-10, -20, -30], -21), 2);
    assert_eq!(cantidad_de_mayores(&[-10, -20, -30], 5), 0);
}
