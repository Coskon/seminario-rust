#![allow(unused)]
pub fn ordenar_nombres<const N: usize>(arr: &mut [String; N]) {
    if !arr.is_empty() {
        for i in 0..N-1 {
            for j in 0..N-i-1 {
                if arr[j] > arr[j+1] {
                    arr.swap(j, j+1);
                }
            }
        }
    }
}

#[test]
fn test_ordenar_nombres() {
    let mut arr1 = ["pablo".to_string(), "juan".to_string(), "pedro".to_string(), "maria".to_string()];
    ordenar_nombres(&mut arr1);
    assert_eq!(arr1, ["juan".to_string(), "maria".to_string(), "pablo".to_string(), "pedro".to_string()]);

    let mut arr2 = ["ad".to_string(), "ab".to_string(), "ac".to_string(), "aa".to_string()];
    ordenar_nombres(&mut arr2);
    assert_eq!(arr2, ["aa".to_string(), "ab".to_string(), "ac".to_string(), "ad".to_string()]);

    let mut arr3 = [];
    ordenar_nombres(&mut arr3);
    assert_eq!(arr3, [] as [String; 0]);

    let mut arr4 = ["Aa".to_string(), "".to_string(), "aA".to_string()];
    ordenar_nombres(&mut arr4);
    assert_eq!(arr4, ["".to_string(), "Aa".to_string(), "aA".to_string()]);
}