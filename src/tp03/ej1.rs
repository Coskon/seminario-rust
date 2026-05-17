#![allow(unused)]
pub struct Persona {
    pub nombre: String,
    pub edad: u8,
    pub direccion: Option<String>
}

impl Persona {
    pub fn new(nombre: &str, edad: u8, direccion: Option<String>) -> Self {
        Persona { nombre: nombre.to_string(), edad, direccion }
    }

    pub fn to_string(&self) -> String {
        match &self.direccion {
            Some(dir) => format!("{}; {}; {}", self.nombre, self.edad, dir),
            _ => format!("{}; {}", self.nombre, self.edad)
        }
    }

    pub fn obtener_edad(&self) -> u8 {
        self.edad
    }

    pub fn actualizar_direccion(&mut self, nueva_direccion: &str) {
        self.direccion = Some(nueva_direccion.to_string());
    }
}


#[test]
fn test_persona_creacion() {
    let p1 = Persona::new("Juan Pedro", 23, None);
    assert_eq!(p1.nombre, "Juan Pedro".to_string());
    assert_eq!(p1.edad, 23 as u8);
    assert_eq!(p1.direccion, None);

    let p2 = Persona::new("Maria", 28, Some("3 y 47 Nro 1111".to_string()));
    assert_eq!(p2.nombre, "Maria".to_string());
    assert_eq!(p2.obtener_edad(), 28 as u8);
    assert_eq!(p2.direccion, Some("3 y 47 Nro 1111".to_string()));
}

#[test]
fn test_persona_actualizar_direccion() {
    let mut p1 = Persona::new("Juan Pedro", 23, None);
    p1.actualizar_direccion("2 y 50 Nro 1234");
    assert_eq!(p1.direccion, Some("2 y 50 Nro 1234".to_string()));
    p1.actualizar_direccion("");
    assert_eq!(p1.direccion, Some("".to_string()));
}

#[test]
fn test_persona_to_string() {
    let p1 = Persona::new("Carlos", 26, None);
    assert_eq!(p1.to_string(), "Carlos; 26".to_string());
    let p2 = Persona::new("Violeta", 22, Some("4 y 49 Nro 3421".to_string()));
    assert_eq!(p2.to_string(), "Violeta; 22; 4 y 49 Nro 3421".to_string());
    let p3 = Persona::new("Ricardo", 24, Some("".to_string()));
    assert_eq!(p3.to_string(), "Ricardo; 24; ".to_string());
}