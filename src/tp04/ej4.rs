#![allow(unused)]
use crate::tp03::ej3::Fecha;
use std::collections::HashMap;
use std::fmt::{Display, Error, Formatter};

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub enum Categoria {
    Ropa, Deportes, Juguetes, Libros, Belleza,
    Comida, Salud, Hogar, Automotor, Electronica
}

impl Categoria {
    fn get_descuento(&self) -> f64 {
        match self {
            Categoria::Ropa => 0.95,
            Categoria::Deportes => 1.0,
            Categoria::Juguetes => 1.0,
            Categoria::Libros => 0.9,
            Categoria::Belleza => 0.95,
            Categoria::Comida => 0.95,
            Categoria::Salud => 1.0,
            Categoria::Hogar => 0.9,
            Categoria::Automotor => 1.0,
            Categoria::Electronica => 1.0
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Producto {
    pub nombre: String,
    pub categoria: Categoria,
    pub precio_base: f64
}

impl Producto {
    pub fn new(nombre: &str, categoria: Categoria, precio_base: f64) -> Result<Self, ErrorSistema> {
        if !precio_base.is_finite() || precio_base <= 0.0 {
            Err(ErrorSistema::PrecioInvalido(precio_base))
        } else {
            Ok(Producto { nombre: nombre.to_string(), categoria, precio_base })
        }
    }

    pub fn new_test_unchecked(nombre: &str, categoria: Categoria, precio_base: f64) -> Self {
        Producto { nombre: nombre.to_string(), categoria, precio_base }
    }

    pub fn get_precio_final(&self) -> f64 {
        self.precio_base * self.categoria.get_descuento()
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Cliente {
    pub nombre: String,
    pub apellido: String,
    pub direccion: String,
    pub email: Option<String>,
    pub dni: u32
}

impl Cliente {
    pub fn new(nombre: &str, apellido: &str, direccion: &str, email: Option<&str>, dni: u32) -> Self {
        Cliente { nombre: nombre.to_string(), apellido: apellido.to_string(), direccion: direccion.to_string(), 
            email: email.map(|x| x.to_string()), dni }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Vendedor {
    pub legajo: u16,
    pub antiguedad: u16,
    pub salario: f64
}

impl Vendedor {
    pub fn new(legajo: u16, antiguedad: u16, salario: f64) -> Self {
        Vendedor { legajo, antiguedad, salario }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum MedioPago {
    TarjetaCredito, TarjetaDebito, Transferencia, Efectivo
}

#[derive(PartialEq, Clone, Debug)]
pub struct Venta {
    fecha: Fecha,
    cliente: Cliente,
    vendedor: Vendedor,
    medio_pago: MedioPago,
    productos: Vec<(Producto, u32)>
}

impl Venta {
    pub fn new(fecha: Fecha, cliente: Cliente, vendedor: Vendedor, medio_pago: MedioPago, productos: Vec<(Producto, u32)>) -> Self {
        Venta { fecha, cliente, vendedor, medio_pago, productos }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum ErrorSistema {
    ListaProductosVacia, PrecioInvalido(f64), CantidadCero(String)
}

impl Display for ErrorSistema {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorSistema::ListaProductosVacia => write!(f, "Lista de productos vacia"),
            ErrorSistema::PrecioInvalido(p) => write!(f, "Precio {} es invalido (cero, negativo, infinito o NaN)", p),
            ErrorSistema::CantidadCero(nombrep) => write!(f, "Cantidad del producto {} es 0", nombrep),
        }
    }
}

pub struct SistemaVentas {
    ventas: Vec<Venta>,
    vendedores: HashMap<u16, Vendedor>,
}

impl SistemaVentas {
    pub fn new() -> Self {
        SistemaVentas { ventas: vec![], vendedores: HashMap::new() }
    }

    pub fn registrar_venta(&mut self, fecha: Fecha, cliente: Cliente, vendedor: Vendedor, 
        medio_pago: MedioPago, productos: Vec<(Producto, u32)>) -> Result<(), ErrorSistema> {
        if productos.is_empty() {
            Err(ErrorSistema::ListaProductosVacia)
        } else {
            productos.iter().try_for_each(|(p, cant)| {
                if !p.precio_base.is_finite() || p.precio_base <= 0.0 { // aunque Producto::new() haga la comprobacion, se puede usar Producto { ... } y poner precio negativo
                    Err(ErrorSistema::PrecioInvalido(p.precio_base))
                } else if *cant == 0 {
                    Err(ErrorSistema::CantidadCero(p.nombre.clone()))
                } else {
                    Ok(())
                }
            })?;
            let venta = Venta::new(fecha, cliente, vendedor.clone(), medio_pago, productos);
            self.vendedores.entry(vendedor.legajo).or_insert(vendedor);

            self.ventas.push(venta);
            Ok(())
        }
    }

    fn get_descuento_newsteller(&self, cliente: &Cliente) -> f64 {
        if cliente.email.is_some() { 0.9 } // 10% si esta suscrito
        else { 1.0 } // sin descuento si no esta suscrito
    }

    fn calcular_precio_final(&self, prod: &Producto, cant: u32) -> f64 {
        prod.get_precio_final()*f64::from(cant)
    }

    pub fn calcular_precio_total(&self, venta: &Venta) -> f64 {
        let precio_prods: f64 = venta.productos.iter()
            .map(|(p, cant)| self.calcular_precio_final(p, *cant))
            .sum();
        precio_prods*self.get_descuento_newsteller(&venta.cliente)
    }

    pub fn get_venta(&self, index: usize) -> Option<&Venta> {
        self.ventas.get(index)
    }

    pub fn get_vendedor(&self, legajo: u16) -> Option<&Vendedor> {
        self.vendedores.get(&legajo)
    }

    pub fn reporte_ventas_categoria(&self) -> HashMap<Categoria, f64> {
        let mut reporte = HashMap::new();

        self.ventas.iter().for_each(|v| {
            let descuento = self.get_descuento_newsteller(&v.cliente);
            v.productos.iter().for_each(|(p, cant)|
                *reporte.entry(p.categoria.clone()).or_insert(0.0) += self.calcular_precio_final(p, *cant)*descuento
            )
        });

        reporte
    }

    pub fn reporte_ventas_vendedor(&self) -> HashMap<u16, f64> {
        let mut reporte = HashMap::new();

        self.ventas.iter().for_each(|v|
            *reporte.entry(v.vendedor.legajo).or_insert(0.0) += self.calcular_precio_total(v)
        );

        reporte
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    
    fn crear_sistema_base() -> SistemaVentas {
        let mut sv = SistemaVentas::new();
        assert!(sv.registrar_venta(
            Fecha::new(1, 2, 2026),
            Cliente::new("Pedro", "Perez", "7 y 47", Some("pedro.perez@gmail.com"), 43_298_371), 
            Vendedor::new(1234, 6, 900_000.0), 
            MedioPago::Efectivo, 
            vec![
                (Producto::new("Arroz 1Kg", Categoria::Comida, 1050.0).unwrap(), 2),
                (Producto::new("Fideos Tallarin", Categoria::Comida, 1250.0).unwrap(), 4),
                (Producto::new("Cargador Celular", Categoria::Electronica, 5649.99).unwrap(), 1),
                (Producto::new("Par Medias", Categoria::Ropa, 3999.99).unwrap(), 1),
            ]).is_ok());
        assert!(sv.registrar_venta(
            Fecha::new(26, 5, 2026),
            Cliente::new("Maria", "Sanchez", "4 y 42", None, 39_876_019), 
            Vendedor::new(1234, 6, 900_000.0), 
            MedioPago::Efectivo, 
            vec![
                (Producto::new("Queso Crema", Categoria::Comida, 2349.99).unwrap(), 2),
                (Producto::new("Aceite Auto", Categoria::Automotor, 44499.99).unwrap(), 1),
                (Producto::new("36u Bloques Construccion", Categoria::Juguetes, 30000.0).unwrap(), 1),
            ]).is_ok());
        assert!(sv.registrar_venta(
            Fecha::new(3, 6, 2026),
            Cliente::new("Juan", "Martinez", "2 y 37", Some("juan.martinez@gmail.com"), 44_414_141), 
            Vendedor::new(5678, 15, 1_200_000.0), 
            MedioPago::TarjetaDebito, 
            vec![
                (Producto::new("Desodorante", Categoria::Belleza, 6500.0).unwrap(), 1),
                (Producto::new("Mancuerna 5kg", Categoria::Deportes, 14999.99).unwrap(), 2),
                (Producto::new("Harry Potter", Categoria::Libros, 34999.99).unwrap(), 1),
                (Producto::new("Lampara LED", Categoria::Hogar, 7000.0).unwrap(), 1)
            ]).is_ok());

        sv
    }

    #[test]
    fn creacion_producto_valido() {
        assert!(Producto::new("Desodorante", Categoria::Belleza, 6500.0).is_ok_and(
            |p| p.nombre == "Desodorante" && p.categoria == Categoria::Belleza && p.precio_base == 6500.0 && p.get_precio_final() == 6500.0 * 0.95));
        assert!(Producto::new("Par Medias", Categoria::Ropa, 3999.99).is_ok_and(
            |p| p.nombre == "Par Medias" && p.categoria == Categoria::Ropa && p.precio_base == 3999.99 && p.get_precio_final() == 3999.99 * 0.95));
    }

    #[test]
    fn creacion_producto_precio_invalido() {
        assert!(Producto::new("Par Medias", Categoria::Ropa, 0.0).is_err_and(|e| e == ErrorSistema::PrecioInvalido(0.0)));
        assert!(Producto::new("Par Medias", Categoria::Ropa, -15.56).is_err_and(|e| e == ErrorSistema::PrecioInvalido(-15.56)));
        assert!(Producto::new("Par Medias", Categoria::Ropa, 0.0_f64.next_down()).is_err_and(|e| e == ErrorSistema::PrecioInvalido(0.0_f64.next_down())));
        assert!(Producto::new("Par Medias", Categoria::Ropa, f64::INFINITY).is_err_and(|e| e == ErrorSistema::PrecioInvalido(f64::INFINITY)));
        assert!(Producto::new("Par Medias", Categoria::Ropa, f64::NEG_INFINITY).is_err_and(|e| e == ErrorSistema::PrecioInvalido(f64::NEG_INFINITY)));
        assert!(Producto::new("Par Medias", Categoria::Ropa, f64::NAN).is_err_and(|e| matches!(e, ErrorSistema::PrecioInvalido(p) if p.is_nan())));
    }

    #[test]
    fn test_crear_venta() {
        let mut sv = SistemaVentas::new();
        assert!(sv.registrar_venta(
            Fecha::new(1, 2, 2026),
            Cliente::new("Pedro", "Perez", "7 y 47", Some("pedro.perez@gmail.com"), 43_298_371), 
            Vendedor::new(1234, 6, 900_000.0), 
            MedioPago::Efectivo, 
            vec![ 
                (Producto::new("Arroz 1Kg", Categoria::Comida, 1050.0).unwrap(), 2),
                (Producto::new("Fideos Tallarin", Categoria::Comida, 1250.0).unwrap(), 4),
                (Producto::new("Cargador Celular", Categoria::Electronica, 5649.99).unwrap(), 1),
                (Producto::new("Par Medias", Categoria::Ropa, 3999.99).unwrap(), 1),
            ]).is_ok());
        assert!(sv.registrar_venta(
            Fecha::new(26, 5, 2026),
            Cliente::new("Maria", "Sanchez", "4 y 42", None, 39_876_019), 
            Vendedor::new(1234, 6, 900_000.0), 
            MedioPago::Efectivo, 
            vec![ 
                (Producto::new("Queso Crema", Categoria::Comida, 2349.99).unwrap(), 2),
                (Producto::new("Aceite Auto", Categoria::Automotor, 44499.99).unwrap(), 1),
                (Producto::new("36u Bloques Construccion", Categoria::Juguetes, 30000.0).unwrap(), 1),
            ]).is_ok());
        assert!(sv.registrar_venta(
            Fecha::new(3, 6, 2026),
            Cliente::new("Juan", "Martinez", "2 y 37", Some("juan.martinez@gmail.com"), 44_414_141), 
            Vendedor::new(5678, 15, 1_200_000.0), 
            MedioPago::TarjetaDebito, 
            vec![
                (Producto::new("Desodorante", Categoria::Belleza, 6500.0).unwrap(), 1),
                (Producto::new("Mancuerna 5kg", Categoria::Deportes, 14999.99).unwrap(), 2),
                (Producto::new("Harry Potter", Categoria::Libros, 34999.99).unwrap(), 1),
                (Producto::new("Lampara LED", Categoria::Hogar, 7000.0).unwrap(), 1),
                (Producto::new("Suplemento Vitamina C", Categoria::Salud, 15000.0).unwrap(), 1),
            ]).is_ok());
        assert!(sv.get_venta(0).is_some_and(|v| v.cliente.dni == 43_298_371 && v.productos.len() == 4));
        assert!(sv.get_venta(1).is_some_and(|v| v.cliente.dni == 39_876_019 && v.productos.len() == 3));
        assert!(sv.get_venta(2).is_some_and(|v| v.cliente.dni == 44_414_141 && v.productos.len() == 5));
        assert!(sv.get_venta(3).is_none());

        assert!(sv.get_vendedor(1234).is_some());
        assert!(sv.get_vendedor(5678).is_some());
        assert!(sv.get_vendedor(2345).is_none());
    }

    #[test]
    fn test_crear_venta_error() {
        let mut sv = SistemaVentas::new();
        assert!(sv.registrar_venta(
            Fecha::new(1, 2, 2026),
            Cliente::new("Pedro", "Perez", "7 y 47", Some("pedro.perez@gmail.com"), 43_298_371), 
            Vendedor::new(1234, 6, 900_000.0), 
            MedioPago::Efectivo, 
            vec![]).is_err_and(|e| e == ErrorSistema::ListaProductosVacia));

        assert!(sv.registrar_venta(
            Fecha::new(1, 2, 2026),
            Cliente::new("Pedro", "Perez", "7 y 47", Some("pedro.perez@gmail.com"), 43_298_371), 
            Vendedor::new(1234, 6, 900_000.0), 
            MedioPago::Efectivo, 
            vec![
                (Producto::new_test_unchecked("Arroz 1Kg", Categoria::Comida, 1050.0), 2),
                (Producto::new_test_unchecked("Fideos Tallarin", Categoria::Comida, -1250.0), 4)
            ]).is_err_and(|e| e == ErrorSistema::PrecioInvalido(-1250.0)));
        assert!(sv.registrar_venta(
            Fecha::new(1, 2, 2026),
            Cliente::new("Pedro", "Perez", "7 y 47", Some("pedro.perez@gmail.com"), 43_298_371), 
            Vendedor::new(1234, 6, 900_000.0), 
            MedioPago::Efectivo, 
            vec![
                (Producto::new_test_unchecked("Arroz 1Kg", Categoria::Comida, 0.0), 2),
                (Producto::new_test_unchecked("Fideos Tallarin", Categoria::Comida, 1250.0), 4)
            ]).is_err_and(|e| e == ErrorSistema::PrecioInvalido(0.0)));

        assert!(sv.registrar_venta(
            Fecha::new(1, 2, 2026),
            Cliente::new("Pedro", "Perez", "7 y 47", Some("pedro.perez@gmail.com"), 43_298_371), 
            Vendedor::new(1234, 6, 900_000.0), 
            MedioPago::Efectivo, 
            vec![
                (Producto::new_test_unchecked("Arroz 1Kg", Categoria::Comida, 500.0), 4),
                (Producto::new_test_unchecked("Fideos Tallarin", Categoria::Comida, f64::INFINITY), 1)
            ]).is_err_and(|e| e == ErrorSistema::PrecioInvalido(f64::INFINITY)));

        assert!(sv.registrar_venta(
            Fecha::new(1, 2, 2026),
            Cliente::new("Pedro", "Perez", "7 y 47", Some("pedro.perez@gmail.com"), 43_298_371), 
            Vendedor::new(1234, 6, 900_000.0), 
            MedioPago::Efectivo, 
            vec![
                (Producto::new_test_unchecked("Arroz 1Kg", Categoria::Comida, 250.0), 3),
                (Producto::new_test_unchecked("Fideos Tallarin", Categoria::Comida, f64::NAN), 1)
            ]).is_err_and(|e| matches!(e, ErrorSistema::PrecioInvalido(p) if p.is_nan())));

        assert!(sv.registrar_venta(
            Fecha::new(1, 2, 2026),
            Cliente::new("Pedro", "Perez", "7 y 47", Some("pedro.perez@gmail.com"), 43_298_371), 
            Vendedor::new(1234, 6, 900_000.0), 
            MedioPago::Efectivo, 
            vec![
                (Producto::new_test_unchecked("Arroz 1Kg", Categoria::Comida, 1050.0), 2),
                (Producto::new_test_unchecked("Fideos Tallarin", Categoria::Comida, 1250.0), 0)
            ]).is_err_and(|e| e == ErrorSistema::CantidadCero("Fideos Tallarin".to_string())));
    }

    #[test]
    fn test_calcular_precio_total() {
        let mut sv = crear_sistema_base();
        assert_eq!(sv.calcular_precio_total(sv.get_venta(0).unwrap()), (1050.0*2.0*0.95+1250.0*4.0*0.95+5649.99+3999.99*0.95)*0.9);
        assert_eq!(sv.calcular_precio_total(sv.get_venta(1).unwrap()), 2349.99*0.95*2.0+44499.99+30000.0);
        assert_eq!(sv.calcular_precio_total(sv.get_venta(2).unwrap()), (6500.0*0.95+14999.99*2.0+34999.99*0.9+7000.0*0.9)*0.9);
    }

    #[test]
    fn test_reporte_ventas_categoria() {
        let mut sv = crear_sistema_base();
        let reporte = sv.reporte_ventas_categoria();
        assert_eq!(reporte.len(), 9);
        assert!(reporte.get(&Categoria::Automotor).is_some_and(|v| *v == 44499.99));
        assert!(reporte.get(&Categoria::Belleza).is_some_and(|v| *v == 6500.0*0.95*0.9));
        assert!(reporte.get(&Categoria::Comida).is_some_and(|v| *v == (1050.0*0.95*2.0+1250.0*0.95*4.0)*0.9+2349.99*0.95*2.0));
        assert!(reporte.get(&Categoria::Deportes).is_some_and(|v| *v == 14999.99*2.0*0.9));
        assert!(reporte.get(&Categoria::Electronica).is_some_and(|v| *v == 5649.99*0.9));
        assert!(reporte.get(&Categoria::Hogar).is_some_and(|v| *v == 7000.0*0.9*0.9));
        assert!(reporte.get(&Categoria::Juguetes).is_some_and(|v| *v == 30000.0));
        assert!(reporte.get(&Categoria::Libros).is_some_and(|v| *v == 34999.99*0.9*0.9));
        assert!(reporte.get(&Categoria::Ropa).is_some_and(|v| *v == 3999.99*0.95*0.9));
        assert!(!reporte.contains_key(&Categoria::Salud));

        sv.registrar_venta(
            Fecha::new(1, 1, 2026), 
            Cliente::new("", "", "", Some(""), 123), 
            Vendedor::new(5678, 15, 1_200_000.0),
            MedioPago::TarjetaDebito,
            vec![
                (Producto::new("Suplemento Vitamina C", Categoria::Salud, 15000.0).unwrap(), 2)
            ]);
        let reporte = sv.reporte_ventas_categoria();
        assert_eq!(reporte.len(), 10);
        assert!(reporte.get(&Categoria::Salud).is_some_and(|v| *v == 15000.0*2.0*0.9));
    }

    #[test]
    fn test_reporte_ventas_categoria_sin_ventas() {
        let mut sv = SistemaVentas::new();
        let reporte = sv.reporte_ventas_categoria();
        assert_eq!(reporte.len(), 0);
        assert!(!reporte.contains_key(&Categoria::Automotor));
        assert!(!reporte.contains_key(&Categoria::Belleza));
        assert!(!reporte.contains_key(&Categoria::Salud));
    }

    #[test]
    fn test_reporte_ventas_vendedor() {
        let mut sv = crear_sistema_base();
        let reporte = sv.reporte_ventas_vendedor();
        assert_eq!(reporte.len(), 2);
        assert!(reporte.get(&1234).is_some_and(|v| *v == (1050.0*0.95*2.0+1250.0*0.95*4.0+5649.99+3999.99*0.95)*0.9 + (2349.99*0.95*2.0+44499.99+30000.0) ));
        assert!(reporte.get(&5678).is_some_and(|v| *v == (6500.0*0.95+14999.99*2.0+34999.99*0.9+7000.0*0.9)*0.9 ));
        assert!(!reporte.contains_key(&2345));
    }

    #[test]
    fn test_reporte_ventas_vendedor_sin_ventas() {
        let mut sv = SistemaVentas::new();
        let reporte = sv.reporte_ventas_vendedor();
        assert_eq!(reporte.len(), 0);
        assert!(!reporte.contains_key(&1234));
        assert!(!reporte.contains_key(&5678));
    }
}