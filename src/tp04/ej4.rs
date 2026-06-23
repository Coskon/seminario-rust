#![allow(unused)]
use crate::tp03::ej3::Fecha;
use std::collections::HashMap;

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
    pub fn new(nombre: &str, categoria: Categoria, precio_base: f64) -> Self {
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
pub enum ErrorVenta {
    ListaProductosVacia, PrecioCeroNegativo, CantidadCero
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
        medio_pago: MedioPago, productos: Vec<(Producto, u32)>) -> Result<(), ErrorVenta> {
        if productos.is_empty() {
            Err(ErrorVenta::ListaProductosVacia)
        } else {
            productos.iter().try_for_each(|(p, cant)| {
                if p.precio_base <= 0.0 {
                    Err(ErrorVenta::PrecioCeroNegativo)
                } else if *cant == 0 {
                    Err(ErrorVenta::CantidadCero)
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
                (Producto::new("Arroz 1Kg", Categoria::Comida, 1050.0), 2),
                (Producto::new("Fideos Tallarin", Categoria::Comida, 1250.0), 4),
                (Producto::new("Cargador Celular", Categoria::Electronica, 5649.99), 1),
                (Producto::new("Par Medias", Categoria::Ropa, 3999.99), 1),
            ]).is_ok());
        assert!(sv.registrar_venta(
            Fecha::new(26, 5, 2026),
            Cliente::new("Maria", "Sanchez", "4 y 42", None, 39_876_019), 
            Vendedor::new(1234, 6, 900_000.0), 
            MedioPago::Efectivo, 
            vec![
                (Producto::new("Queso Crema", Categoria::Comida, 2349.99), 2),
                (Producto::new("Aceite Auto", Categoria::Automotor, 44499.99), 1),
                (Producto::new("36u Bloques Construccion", Categoria::Juguetes, 30000.0), 1),
            ]).is_ok());
        assert!(sv.registrar_venta(
            Fecha::new(3, 6, 2026),
            Cliente::new("Juan", "Martinez", "2 y 37", Some("juan.martinez@gmail.com"), 44_414_141), 
            Vendedor::new(5678, 15, 1_200_000.0), 
            MedioPago::TarjetaDebito, 
            vec![
                (Producto::new("Desodorante", Categoria::Belleza, 6500.0), 1),
                (Producto::new("Mancuerna 5kg", Categoria::Deportes, 14999.99), 2),
                (Producto::new("Harry Potter", Categoria::Libros, 34999.99), 1),
                (Producto::new("Lampara LED", Categoria::Hogar, 7000.0), 1)
            ]).is_ok());

        sv
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
                (Producto::new("Arroz 1Kg", Categoria::Comida, 1050.0), 2),
                (Producto::new("Fideos Tallarin", Categoria::Comida, 1250.0), 4),
                (Producto::new("Cargador Celular", Categoria::Electronica, 5649.99), 1),
                (Producto::new("Par Medias", Categoria::Ropa, 3999.99), 1),
            ]).is_ok());
        assert!(sv.registrar_venta(
            Fecha::new(26, 5, 2026),
            Cliente::new("Maria", "Sanchez", "4 y 42", None, 39_876_019), 
            Vendedor::new(1234, 6, 900_000.0), 
            MedioPago::Efectivo, 
            vec![ 
                (Producto::new("Queso Crema", Categoria::Comida, 2349.99), 2),
                (Producto::new("Aceite Auto", Categoria::Automotor, 44499.99), 1),
                (Producto::new("36u Bloques Construccion", Categoria::Juguetes, 30000.0), 1),
            ]).is_ok());
        assert!(sv.registrar_venta(
            Fecha::new(3, 6, 2026),
            Cliente::new("Juan", "Martinez", "2 y 37", Some("juan.martinez@gmail.com"), 44_414_141), 
            Vendedor::new(5678, 15, 1_200_000.0), 
            MedioPago::TarjetaDebito, 
            vec![
                (Producto::new("Desodorante", Categoria::Belleza, 6500.0), 1),
                (Producto::new("Mancuerna 5kg", Categoria::Deportes, 14999.99), 2),
                (Producto::new("Harry Potter", Categoria::Libros, 34999.99), 1),
                (Producto::new("Lampara LED", Categoria::Hogar, 7000.0), 1),
                (Producto::new("Suplemento Vitamina C", Categoria::Salud, 15000.0), 1),
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
            vec![]).is_err_and(|e| e == ErrorVenta::ListaProductosVacia));

        assert!(sv.registrar_venta(
            Fecha::new(1, 2, 2026),
            Cliente::new("Pedro", "Perez", "7 y 47", Some("pedro.perez@gmail.com"), 43_298_371), 
            Vendedor::new(1234, 6, 900_000.0), 
            MedioPago::Efectivo, 
            vec![
                (Producto::new("Arroz 1Kg", Categoria::Comida, 1050.0), 2),
                (Producto::new("Fideos Tallarin", Categoria::Comida, -1250.0), 4)
            ]).is_err_and(|e| e == ErrorVenta::PrecioCeroNegativo));
        assert!(sv.registrar_venta(
            Fecha::new(1, 2, 2026),
            Cliente::new("Pedro", "Perez", "7 y 47", Some("pedro.perez@gmail.com"), 43_298_371), 
            Vendedor::new(1234, 6, 900_000.0), 
            MedioPago::Efectivo, 
            vec![
                (Producto::new("Arroz 1Kg", Categoria::Comida, 0.0), 2),
                (Producto::new("Fideos Tallarin", Categoria::Comida, 1250.0), 4)
            ]).is_err_and(|e| e == ErrorVenta::PrecioCeroNegativo));

        assert!(sv.registrar_venta(
            Fecha::new(1, 2, 2026),
            Cliente::new("Pedro", "Perez", "7 y 47", Some("pedro.perez@gmail.com"), 43_298_371), 
            Vendedor::new(1234, 6, 900_000.0), 
            MedioPago::Efectivo, 
            vec![
                (Producto::new("Arroz 1Kg", Categoria::Comida, 1050.0), 2),
                (Producto::new("Fideos Tallarin", Categoria::Comida, 1250.0), 0)
            ]).is_err_and(|e| e == ErrorVenta::CantidadCero));
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
                (Producto::new("Suplemento Vitamina C", Categoria::Salud, 15000.0), 2)
            ]);
        let reporte = sv.reporte_ventas_categoria();
        assert_eq!(reporte.len(), 10);
        assert!(reporte.get(&Categoria::Salud).is_some_and(|v| *v == 15000.0*2.0*0.9));
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
}