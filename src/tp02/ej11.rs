#![allow(unused)]
pub fn multiplicar_valores<const N: usize>(arr: &mut [i32; N], factor: i32) {
    for n in arr {
        *n *= factor;
    }
}

#[test]
fn test_multiplicar_valores() {
    let mut arr1 = [1, 2, 3, 4];
    multiplicar_valores(&mut arr1, 2);
    assert_eq!(arr1, [2, 4, 6, 8]);
    multiplicar_valores(&mut arr1, -4);
    assert_eq!(arr1, [-8, -16, -24, -32]);
    multiplicar_valores(&mut arr1, 0);
    assert_eq!(arr1, [0, 0, 0, 0]);

    let mut arr2 = [];
    multiplicar_valores(&mut arr2, 2);
    assert_eq!(arr2, []);
    multiplicar_valores(&mut arr2, -4);
    assert_eq!(arr2, []);
    multiplicar_valores(&mut arr2, 0);
    assert_eq!(arr2, []);

    let mut arr3 = [0, 0, 0];
    multiplicar_valores(&mut arr3, 2);
    assert_eq!(arr3, [0, 0, 0]);
    multiplicar_valores(&mut arr3, i32::MAX);
    assert_eq!(arr3, [0, 0, 0]);
    multiplicar_valores(&mut arr3, i32::MIN);
    assert_eq!(arr3, [0, 0, 0]);
}