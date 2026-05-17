#![allow(unused)]
pub fn es_par(n: i32) -> bool {
    n % 2 == 0
}

#[test]
fn test_es_par() {
    assert!(es_par(2));
    assert!(es_par(0));
    assert!(es_par(-80));
    assert!(!es_par(-5));
    assert!(!es_par(3));
    assert!(es_par(i32::MIN)); // -2147483648
    assert!(!es_par(i32::MAX)); // 2147483647
}