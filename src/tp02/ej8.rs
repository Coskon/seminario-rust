#![allow(unused)]
pub fn sumar_arreglos<const N: usize>(mut arr1: [f32; N], arr2: [f32; N]) -> [f32; N] {
    for i in 0..N {
        arr1[i] += arr2[i];
    }
    arr1
}

#[test]
fn test_sumar_arreglos() {
    assert_eq!(sumar_arreglos([0.0, 0.0, 0.0], [0.0, 0.0, 0.0]), [0.0, 0.0, 0.0]);
    assert_eq!(sumar_arreglos([], []), []);
    assert_eq!(sumar_arreglos([-1.0, 1.0], [1.0, -1.0]), [0.0, 0.0]);
    assert_eq!(sumar_arreglos([1.0, 2.0, 3.0], [5.0, 6.0, 7.0]), [6.0, 8.0, 10.0]);
    assert_eq!(sumar_arreglos([-10.5, 5.0, 3.14], [15.0, 2.5, -0.14]), [4.5, 7.5, 3.0]);
}