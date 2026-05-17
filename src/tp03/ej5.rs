#![allow(unused)]
pub struct Producto {
    pub nombre: String,
    pub precio_bruto: f64,
    pub id: u64
}

impl Producto {
    pub fn new(nombre: &str, precio_bruto: f64, id: u64) -> Self {
        assert!(precio_bruto >= 0.0, "El precio no puede ser negativo");
        Producto { nombre: nombre.to_string(), precio_bruto, id }
    }

    pub fn calcular_impuestos(&self, porcentaje: f64) -> f64 {
        assert!(porcentaje >= 0.0, "El porcentaje debe ser positivo");
        self.precio_bruto * porcentaje / 100.0
    }

    pub fn aplicar_descuento(&self, porcentaje: f64) -> f64 {
        assert!(porcentaje >= 0.0, "El porcentaje debe ser positivo");
        assert!(porcentaje <= 100.0, "El porcentaje debe ser menor o igual a 100");
        self.precio_bruto * porcentaje / 100.0
    }

    pub fn calcular_precio_total(&self, porcentaje_impuestos: Option<f64>, porcentaje_descuento: Option<f64>) -> f64 {
        let mut precio_total = self.precio_bruto;
        if let Some(pimp) = porcentaje_impuestos {
            precio_total += self.calcular_impuestos(pimp);
        }
        if let Some(pdesc) = porcentaje_descuento {
            precio_total -= self.aplicar_descuento(pdesc);
        }
        precio_total
    }
}


#[should_panic(expected="El precio no puede ser negativo")]
#[test]
fn test_producto_creacion_invalido() {
    Producto::new("invalido", -1.0, 1);
}

#[test]
fn test_producto_impuestos() {
    let p = Producto::new("coca cola", 2500.0, 1);
    assert_eq!(p.calcular_impuestos(21.0), 525.0);

    let p = Producto::new("pepsi", 2000.0, 2);
    assert_eq!(p.calcular_impuestos(0.0), 0.0);

    let p = Producto::new("cunnington", 1800.0, 3);
    assert_eq!(p.calcular_impuestos(100.0), 1800.0);
}

#[should_panic(expected="El porcentaje debe ser positivo")]
#[test]
fn test_producto_impuestos_negativo() {
    let p = Producto::new("coca cola", 2500.0, 1);
    p.calcular_impuestos(-21.0);
}

#[test]
fn test_producto_descuento() {
    let p = Producto::new("coca cola", 2500.0, 1);
    assert_eq!(p.aplicar_descuento(10.0), 250.0);

    let p = Producto::new("pepsi", 2000.0, 2);
    assert_eq!(p.aplicar_descuento(0.0), 0.0);

    let p = Producto::new("cunnington", 1800.0, 3);
    assert_eq!(p.aplicar_descuento(100.0), 1800.0);
}

#[should_panic(expected="El porcentaje debe ser positivo")]
#[test]
fn test_producto_descuento_negativo() {
    let p = Producto::new("coca cola", 2500.0, 1);
    p.aplicar_descuento(-21.0);
}
#[should_panic(expected="El porcentaje debe ser menor o igual a 100")]
#[test]
fn test_producto_descuento_mayor_100() {
    let p = Producto::new("coca cola", 2500.0, 1);
    p.aplicar_descuento(150.0);
}

#[test]
fn test_producto_precio_total() {
    let p = Producto::new("coca cola", 2500.0, 1);
    assert_eq!(p.calcular_precio_total(Some(21.0), Some(10.0)), 2775.0);

    let p = Producto::new("coca cola", 2500.0, 1);
    assert_eq!(p.calcular_precio_total(None, None), 2500.0);

    let p = Producto::new("coca cola", 2500.0, 1);
    assert_eq!(p.calcular_precio_total(Some(21.0), None), 3025.0);

    let p = Producto::new("coca cola", 2500.0, 1);
    assert_eq!(p.calcular_precio_total(None, Some(10.0)), 2250.0);
}

#[should_panic(expected="El porcentaje debe ser positivo")]
#[test]
fn test_producto_precio_total_impuestos_negativo() {
    let p = Producto::new("coca cola", 2500.0, 1);
    p.calcular_precio_total(Some(-21.0), None);
}
#[should_panic(expected="El porcentaje debe ser positivo")]
#[test]
fn test_producto_precio_total_descuento_negativo() {
    let p = Producto::new("coca cola", 2500.0, 1);
    p.calcular_precio_total(None, Some(-10.0));
}
#[should_panic(expected="El porcentaje debe ser menor o igual a 100")]
#[test]
fn test_producto_precio_total_descuento_mayor_100() {
    let p = Producto::new("coca cola", 2500.0, 1);
    p.calcular_precio_total(Some(21.0), Some(150.0));
}