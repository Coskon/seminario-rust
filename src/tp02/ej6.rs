#![allow(unused)]
pub fn longitud_de_cadenas<const N: usize>(arr: [String; N]) -> [i32; N] {
    let mut newarr: [i32; N] = [0; N];
    for i in 0..N {
        newarr[i] = arr[i].len() as i32;
    }
    newarr
}

#[test]
fn test_longitud_de_cadenas() {
    assert_eq!(longitud_de_cadenas(["a".to_string(), "bb".to_string(), "ccc".to_string(), "dddd".to_string(), "eeeee".to_string()]), [1, 2, 3, 4, 5]);
    assert_eq!(longitud_de_cadenas(["".to_string(), String::new(), "".to_string(), String::from(""), "".to_string()]), [0, 0, 0, 0, 0]);
    assert_eq!(longitud_de_cadenas(["abc".to_string(), "".to_string(), "hello".to_string(), String::new(), "wor".to_string()]), [3, 0, 5, 0, 3]);
    assert_ne!(longitud_de_cadenas(["".to_string(), "1".to_string(), " 2".to_string(), "  3".to_string(), "0".to_string()]), [1, 1, 1, 1, 1]);
}