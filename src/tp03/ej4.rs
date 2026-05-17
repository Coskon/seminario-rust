#![allow(unused)]
pub struct Triangulo {
    pub a: f64,
    pub b: f64,
    pub c: f64
}

impl Triangulo {
    pub fn new(a: f64, b: f64, c: f64) -> Self {
        if a <= 0.0 || b <= 0.0 || c <= 0.0 {
            panic!("Lados del triangulo deben ser mayor a 0")
        }
        let (min1, min2, max) = Triangulo::valores_ordenados(a, b, c);
        if max >= min1 + min2 {
            panic!("Lado mayor debe ser menor que la suma de los 2 menores");
        }
        Triangulo { a, b, c }
    }

    fn valores_ordenados(a: f64, b: f64, c: f64) -> (f64, f64, f64) {
        let mut vals = [a, b, c];
        vals.sort_by(|a, b| a.total_cmp(b)); // vals.sort() no sirve por NaN =(
        (vals[0], vals[1], vals[2])
    }

    pub fn determinar_tipo(&self) -> &str {
        if self.a == self.b && self.b == self.c {
            "equilatero"
        } else if (self.a == self.b) || (self.a == self.c) || (self.b == self.c) {
            "isosceles"
        } else {
            "escaleno"
        }
    }

    pub fn calcular_area(&self) -> f64 {
        let s = (self.a+self.b+self.c)/2.0;
        (s*(s-self.a)*(s-self.b)*(s-self.c)).sqrt()
    }

    pub fn calcular_perimetro(&self) -> f64 {
        self.a + self.b + self.c
    }
}

#[should_panic(expected="Lados del triangulo deben ser mayor a 0")]
#[test]
fn test_triangulo_creacion_negativo_a() {
    Triangulo::new(-1.0, 0.1, 0.0);
}
#[should_panic(expected="Lados del triangulo deben ser mayor a 0")]
#[test]
fn test_triangulo_creacion_negativo_b() {
    Triangulo::new(0.5, 0.0, 0.3);
}
#[should_panic(expected="Lados del triangulo deben ser mayor a 0")]
#[test]
fn test_triangulo_creacion_negativo_c() {
    Triangulo::new(0.1, 0.2, -0.1);
}
#[should_panic(expected="Lado mayor debe ser menor que la suma de los 2 menores")]
#[test]
fn test_triangulo_creacion_invalida() {
    Triangulo::new(1.0, 2.0, 1.0);
}


#[test]
fn test_triangulo_tipo() {
    let t = Triangulo::new(1.0, 1.0, 1.0);
    assert_eq!(t.determinar_tipo(), "equilatero");

    let t = Triangulo::new(2.0, 2.0, 1.0);
    assert_eq!(t.determinar_tipo(), "isosceles");

    let t = Triangulo::new(1.0, 2.0, 2.0);
    assert_eq!(t.determinar_tipo(), "isosceles");

    let t = Triangulo::new(2.0, 1.0, 2.0);
    assert_eq!(t.determinar_tipo(), "isosceles");

    let t = Triangulo::new(1.4, 2.5, 3.6);
    assert_eq!(t.determinar_tipo(), "escaleno");
}

#[test]
fn test_triangulo_area() {
    let t = Triangulo::new(1.0, 1.0, 1.0);
    assert_eq!(t.calcular_area(), 0.1875_f64.sqrt());

    let t = Triangulo::new(2.0, 3.0, 4.0);
    assert_eq!(t.calcular_area(), (4.5*(4.5-2.0)*(4.5-3.0)*(4.5-4.0) as f64).sqrt());

    let t = Triangulo::new(1.5, 2.0, 1.0);
    assert_eq!(t.calcular_area(), (2.25*(2.25-1.5)*(2.25-2.0)*(2.25-1.0) as f64).sqrt());
}

#[test]
fn test_triangulo_perimetro() {
    let t = Triangulo::new(1.0, 1.0, 1.0);
    assert_eq!(t.calcular_perimetro(), 3.0);

    let t = Triangulo::new(2.0, 3.0, 4.0);
    assert_eq!(t.calcular_perimetro(), 9.0);

    let t = Triangulo::new(1.5, 2.0, 1.0);
    assert_eq!(t.calcular_perimetro(), 4.5);
}