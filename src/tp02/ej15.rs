#![allow(unused)]
pub fn serie_geometrica<const TAMAÑO: usize>() -> [u32; TAMAÑO] {
    let mut arr: [u32; TAMAÑO] = [0; TAMAÑO];
    if !arr.is_empty() {
        arr[0] = 1;
        for i in 1..TAMAÑO {
            arr[i] = arr[i-1]*2;
        }
    }
    arr
}

#[test]
fn test_serie_geometrica() {
    assert_eq!(serie_geometrica::<1>(), [1]);
    assert_eq!(serie_geometrica::<2>(), [1, 2]);
    assert_eq!(serie_geometrica::<3>(), [1, 2, 4]);
    assert_eq!(serie_geometrica::<5>(), [1, 2, 4, 8, 16]);
    assert_eq!(serie_geometrica::<0>(), []);
}