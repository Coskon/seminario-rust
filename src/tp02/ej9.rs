#![allow(unused)]
pub fn cantidad_en_rango<const N: usize>(arr: [i32; N], inferior: i32, superior: i32) -> i32 {
    if superior < inferior {
        panic!("Limite superior debe ser mayor o igual a limite inferior.");
    }
    let mut cant = 0;
    for n in arr {
        if n >= inferior && n <= superior {
            cant += 1;
        }
    }
    cant
}

#[test]
fn test_cantidad_en_rango() {
    assert_eq!(cantidad_en_rango([2, 5, 10, 18, 23], 3, 18), 3);
    assert_eq!(cantidad_en_rango([], 1, 2), 0);
    assert_eq!(cantidad_en_rango([3, -5, 1, 1, 9, 10], 1, 1), 2);
    assert_eq!(cantidad_en_rango([-1, -2, 1, 0], 3, 5), 0);
    assert_eq!(cantidad_en_rango([-10, 10], -100, 100), 2);
    assert_eq!(cantidad_en_rango([-2, -1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10], i32::MIN, i32::MAX), 13);
}

#[test]
#[should_panic]
fn test_cantidad_en_rango_invalid() {
    cantidad_en_rango([1, 2, 3], 3, 1);
}