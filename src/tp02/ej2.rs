#![allow(unused)]
pub fn es_primo(n: u32) -> bool {
    if n < 1 { panic!("Numero invalido (debe ser igual o mayor a 1).") };
    if n == 1 { return false; }
    for i in 2..=n/2 {
        if n % i == 0 {
            return false;
        }
    }
    true
}

#[test]
fn test_es_primo() {
    assert!(!es_primo(1));
    assert!(es_primo(2));
    assert!(es_primo(3));
    assert!(!es_primo(4));
    assert!(es_primo(5));
}

#[test]
#[should_panic]
fn test_es_primo_invalid() {
    es_primo(0);
}