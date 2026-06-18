#![allow(unused)]
pub trait EsPrimo {
    fn es_primo(&self) -> bool;
}

impl EsPrimo for i32 {
    fn es_primo(&self) -> bool {
        if (*self < 0) {
            (-*self).es_primo()
        } else if (*self > 1) {
            for i in 2..=self.isqrt() {
                if *self % i == 0 {
                    return false;
                }
            }
            true
        } else {
            false
        }
    }
}

fn cant_num_primos(v: Vec<i32>) -> usize {
    v.iter().filter(|n| n.es_primo()).count()
}

#[cfg(test)]
mod tests {
    use super::cant_num_primos;

    #[test]
    fn test_cant_num_primos() {
        assert_eq!(cant_num_primos(vec![1, 2, 3, 4, 5, 6]), 3);
        assert_eq!(cant_num_primos(vec![1, 2, 4, 8, 16, 32]), 1);
        assert_eq!(cant_num_primos(vec![11, 13, 17, 19, 23]), 5);
        assert_eq!(cant_num_primos(vec![]), 0);
    }

    #[test]
    fn test_cant_num_primos_neg() {
        assert_eq!(cant_num_primos(vec![-2, -3, -5, -1]), 3);
        assert_eq!(cant_num_primos(vec![-19, 19]), 2);
        assert_eq!(cant_num_primos(vec![-1, 0, 1]), 0);
    }
}