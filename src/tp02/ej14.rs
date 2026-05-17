#![allow(unused)]
pub fn incrementar(num: &mut f32) {
    *num += 1.0;
}

#[test]
fn test_incrementar() {
    let mut n1 = 0.0;
    incrementar(&mut n1);
    assert_eq!(n1, 1.0);

    let mut n2 = -1.5;
    incrementar(&mut n2);
    assert_eq!(n2, -0.5);

    let mut n3 = -1.0;
    incrementar(&mut n3);
    assert_eq!(n3, 0.0);

    let mut n4 = f32::MAX;
    incrementar(&mut n4);
    assert_eq!(n4, f32::MAX);

    let mut n5 = 3.14159;
    incrementar(&mut n5);
    assert_eq!(n5, 4.14159);

    let mut n6 = f32::INFINITY;
    incrementar(&mut n6);
    assert!(n6.is_infinite());

    let mut n7 = f32::NAN;
    incrementar(&mut n7);
    assert!(n7.is_nan());
}