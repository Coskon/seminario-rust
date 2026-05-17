#![allow(unused)]
pub struct Rectangulo {
    pub longitud: f64,
    pub ancho: f64
}

impl Rectangulo {
    pub fn new(longitud: f64, ancho: f64) -> Self {
        if longitud <= 0.0 {
            panic!("La longitud debe ser mayor a 0");
        }
        if ancho <= 0.0 {
            panic!("El ancho debe ser mayor a 0.");
        }
        Rectangulo { longitud, ancho }
    }

    pub fn calcular_area(&self) -> f64 {
        self.longitud * self.ancho
    }

    pub fn calcular_perimetro(&self) -> f64 {
        2.0 * self.longitud + 2.0 * self.ancho
    }

    pub fn es_cuadrado(&self) -> bool {
        self.longitud == self.ancho
    }
}

#[should_panic(expected="La longitud debe ser mayor a 0")]
#[test]
fn test_rectangulo_creacion_longitud_negativa() {
    Rectangulo::new(-1.5, 1.0);
}

#[should_panic(expected="El ancho debe ser mayor a 0")]
#[test]
fn test_rectangulo_creacion_ancho_negativo() {
    Rectangulo::new(1.5, -1.0);
}

#[should_panic(expected="La longitud debe ser mayor a 0")]
#[test]
fn test_rectangulo_creacion_longitud_cero() {
    Rectangulo::new(0.0, 1.0);
}

#[should_panic(expected="El ancho debe ser mayor a 0")]
#[test]
fn test_rectangulo_creacion_ancho_cero() {
    Rectangulo::new(1.5, -0.0);
}

#[test]
fn test_rectangulo_area() {
    let r = Rectangulo::new(3.0, 4.0);
    assert_eq!(r.calcular_area(), 12.0);

    let r = Rectangulo::new(1.45, 3.76);
    assert_eq!(r.calcular_area(), 5.452);

    let r = Rectangulo::new(1.0, 1.0);
    assert_eq!(r.calcular_area(), 1.0);
}

#[test]
fn test_rectangulo_perimetro() {
    let r = Rectangulo::new(3.0, 4.0);
    assert_eq!(r.calcular_perimetro(), 14.0);

    let r = Rectangulo::new(1.45, 3.76);
    assert_eq!(r.calcular_perimetro(), 10.42);

    let r = Rectangulo::new(1.0, 1.0);
    assert_eq!(r.calcular_perimetro(), 4.0);
}

#[test]
fn test_rectangulo_es_cuadrado() {
    let r = Rectangulo::new(3.0, 4.0);
    assert_eq!(r.es_cuadrado(), false);

    let r = Rectangulo::new(1.45, 3.76);
    assert_eq!(r.es_cuadrado(), false);

    let r = Rectangulo::new(1.0, 1.0);
    assert_eq!(r.es_cuadrado(), true);

    let r = Rectangulo::new(f64::MAX, f64::MAX);
    assert_eq!(r.es_cuadrado(), true);
}