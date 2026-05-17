#![allow(unused)]
pub fn duplicar_valores<const N: usize>(arr: [f32; N]) -> [f32; N] {
    let mut newarr = arr.clone();
    for x in &mut newarr {
        *x *= 2.0;
    }
    newarr
}

#[test]
fn test_duplicar_valores() {
    assert_eq!(duplicar_valores([1.0, 2.0, 3.0]), [2.0, 4.0, 6.0]);
    assert_eq!(duplicar_valores([-2.5, 3.14159, 0.0, -10.5, 2.2]), [-5.0, 6.28318, 0.0, -21.0, 4.4]);
    assert_eq!(duplicar_valores([0.0, 0.0, 0.0, 0.0]), [0.0, 0.0, 0.0, 0.0]);
    assert_eq!(duplicar_valores([-0.2, -0.1, 0.0, 0.1, 0.2]), [-0.4, -0.2, 0.0, 0.2, 0.4]);
}