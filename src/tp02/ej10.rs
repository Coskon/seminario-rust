#![allow(unused)]
use super::ej6::longitud_de_cadenas;
use super::ej7::cantidad_de_mayores;

pub fn cantidad_de_cadenas_mayor_a<const N: usize>(arr: [String; N], limite: i32) -> i32 {
    cantidad_de_mayores(&longitud_de_cadenas(arr), limite)
}

#[test]
fn test_cantidad_de_cadenas_mayor_a() {
    assert_eq!(cantidad_de_cadenas_mayor_a(["aa".to_string(), String::from("map"), "".to_string(), "hola".to_string()], 2), 2);
    assert_eq!(cantidad_de_cadenas_mayor_a(["aa".to_string(), String::from("map"), "".to_string(), "hola".to_string()], 10), 0);
    assert_eq!(cantidad_de_cadenas_mayor_a(["aa".to_string(), String::from("map"), "".to_string(), "hola".to_string()], 0), 3);
    assert_eq!(cantidad_de_cadenas_mayor_a([], 2), 0);
    assert_eq!(cantidad_de_cadenas_mayor_a([], -1), 0);
    assert_eq!(cantidad_de_cadenas_mayor_a(["".to_string()], 0), 0);
}