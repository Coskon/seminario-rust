
#![allow(unused)]
#[derive(PartialEq, Debug, Clone)]
pub enum Color {
    ROJO, VERDE, AZUL, AMARILLO, BLANCO, NEGRO
}

impl Color {
    pub fn es_primario(&self) -> bool {
        *self == Color::ROJO || *self == Color::AZUL || *self == Color::AMARILLO
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Auto {
    marca: String,
    modelo: String,
    anio: u64,
    precio_bruto: f64,
    color: Color
}

impl Auto {
    pub fn new(marca: &str, modelo: &str, anio: u64, precio_bruto: f64, color: Color) -> Self {
        Auto { marca: marca.to_string(), modelo: modelo.to_string(), anio, precio_bruto, color }
    }

    pub fn calcular_precio(&self) -> f64 {
        let mut precio_final = self.precio_bruto;
        if self.color.es_primario() {
            precio_final += precio_final * 0.25;
        } else {
            precio_final -= precio_final * 0.1;
        }

        if self.marca.to_lowercase() == "bmw" {
            precio_final += precio_final * 0.15;
        }

        if self.anio < 2000 {
            precio_final -= precio_final * 0.05;
        }

        precio_final
    }
}

pub struct ConcesionarioAuto {
    pub nombre: String,
    pub direccion: String,
    autos: Vec<Auto>,
    capacidad: usize
}

impl ConcesionarioAuto {
    pub fn new(nombre: &str, direccion: &str, autos: Vec<Auto>, capacidad: usize) -> Self {
        ConcesionarioAuto { nombre: nombre.to_string(), direccion: direccion.to_string(), autos, capacidad }
    }

    pub fn agregar_auto(&mut self, auto: Auto) -> bool {
        if self.autos.len() < self.capacidad  {
            self.autos.push(auto);
            true
        } else {
            false
        }
    }

    pub fn buscar_auto(&self, auto: Auto) -> Option<Auto> {
        for au in &self.autos {
            if *au == auto {
                return Some(au.clone());
            }
        }
        None
    }
    
    pub fn eliminar_auto(&mut self, auto: Auto) -> bool {
        if let Some(auto_borrar) = self.buscar_auto(auto) {
            self.autos.swap_remove(self.autos.iter().position(|x| *x == auto_borrar).unwrap());
            true
        } else {
            false
        }
    }
}

#[test]
fn test_color_es_primario() {
    assert!(Color::ROJO.es_primario());
    assert!(!Color::NEGRO.es_primario());
}

#[test]
fn test_auto_calcular_precio() {
    let a = Auto::new("Peugeot", "208", 2006, 10_000_000.0, Color::BLANCO);
    assert_eq!(a.calcular_precio(), 9_000_000.0);

    let a = Auto::new("Peugeot", "206", 1999, 5_000_000.0, Color::AZUL);
    assert_eq!(a.calcular_precio(), 5_937_500.0);

    let a = Auto::new("BMW", "Serie 1", 2012, 75_000_000.0, Color::ROJO);
    assert_eq!(a.calcular_precio(), 107_812_500.0);
}

#[test]
fn test_concesionario_agregar_auto() {
    let mut c = ConcesionarioAuto::new("Concesionario Peugeot", "4 y 50", vec![], 3);
    assert!(c.agregar_auto(Auto::new("Peugeot", "208", 2006, 10_000_000.0, Color::BLANCO)));
    assert!(c.agregar_auto(Auto::new("Peugeot", "206", 1999, 5_000_000.0, Color::ROJO)));
    assert!(c.agregar_auto(Auto::new("Peugeot", "301", 2010, 19_000_000.0, Color::AZUL)));
    assert!(!c.agregar_auto(Auto::new("Peugeot", "306", 2015, 30_000_000.0, Color::NEGRO))); // max capacidad alcanzada
}

#[test]
fn test_concesionario_eliminar_auto() {
    let mut c = ConcesionarioAuto::new("Concesionario Peugeot", "4 y 50", 
    vec![
            Auto::new("Peugeot", "208", 2006, 10_000_000.0, Color::BLANCO),
            Auto::new("Peugeot", "206", 1999, 5_000_000.0, Color::ROJO),
            Auto::new("Peugeot", "301", 2010, 19_000_000.0, Color::AZUL),
            Auto::new("Peugeot", "306", 2015, 30_000_000.0, Color::NEGRO)
        ], 5);
    assert!(c.eliminar_auto(Auto::new("Peugeot", "208", 2006, 10_000_000.0, Color::BLANCO)));
    assert!(!c.eliminar_auto(Auto::new("Peugeot", "206", 1999, 5_000_000.0, Color::AZUL))); // todo igual menos el color
    assert!(!c.eliminar_auto(Auto::new("BMW", "Serie 1", 2012, 75_000_000.0, Color::AMARILLO))) // todo distinto
}

#[test]
fn test_concesionario_buscar_auto() {
    let c = ConcesionarioAuto::new("Concesionario Peugeot", "4 y 50", 
    vec![
            Auto::new("Peugeot", "208", 2006, 10_000_000.0, Color::BLANCO),
            Auto::new("Peugeot", "206", 1999, 5_000_000.0, Color::ROJO),
            Auto::new("Peugeot", "301", 2010, 19_000_000.0, Color::AZUL),
            Auto::new("Peugeot", "306", 2015, 30_000_000.0, Color::NEGRO)
        ], 5);
    assert_ne!(c.buscar_auto(Auto::new("Peugeot", "208", 2006, 10_000_000.0, Color::BLANCO)), None);
    assert_eq!(c.buscar_auto(Auto::new("Peugeot", "206", 1999, 5_000_000.0, Color::AZUL)), None); // todo igual menos el color
    assert_eq!(c.buscar_auto(Auto::new("BMW", "Serie 1", 2012, 75_000_000.0, Color::AMARILLO)), None) // todo distinto
}

