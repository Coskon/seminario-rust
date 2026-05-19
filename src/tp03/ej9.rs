#![allow(unused)]
use super::ej3::Fecha;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub enum Animal {
    PERRO, GATO, CABALLO, OTRO
}

impl Animal {
    pub fn equals(&self, other: &Self) -> bool {
        match (self, other) {
            (Animal::PERRO, Animal::PERRO) => true, (Animal::CABALLO, Animal::CABALLO) => true,
            (Animal::GATO, Animal::GATO) => true, (Animal::OTRO, Animal::OTRO) => true,
            _ => false
        }
    }
}

#[derive(Debug, Clone)]
pub struct Persona {
    pub nombre: String,
    pub direccion: String,
    pub telefono: String
}

impl Persona {
    pub fn new(nombre: &str, direccion: &str, telefono: &str) -> Self {
        Persona { nombre: nombre.to_string(), direccion: direccion.to_string(), telefono: telefono.to_string() }
    }

    pub fn equals(&self, other: &Self) -> bool {
        self.nombre == other.nombre && self.direccion == other.direccion && self.telefono == other.telefono
    }
}

#[derive(Debug, Clone)]
pub struct Mascota {
    pub nombre: String,
    pub edad: u32,
    pub tipo: Animal,
    pub duenio: Persona
}

impl Mascota {
    pub fn new(nombre: &str, edad: u32, tipo: Animal, duenio: Persona) -> Self {
        Mascota { nombre: nombre.to_string(), edad, tipo, duenio }
    }

    pub fn equals(&self, other: &Self) -> bool {
        self.nombre == other.nombre && self.edad == other.edad && self.tipo.equals(&other.tipo) && self.duenio.equals(&other.duenio)
    }

    pub fn equals_reducido(&self, other_nombre: &str, other_nombre_duenio: &str, other_telefono: &str) -> bool {
        self.nombre.as_str() == other_nombre &&
        self.duenio.nombre.as_str() == other_nombre_duenio &&
        self.duenio.telefono.as_str() == other_telefono
    }
}

#[derive(Debug, Clone)]
pub struct Atencion {
    pub mascota: Mascota,
    pub diagnostico: String,
    pub tratamiento: String,
    pub proxima_visita: Option<Fecha>
}

impl Atencion {
    pub fn new(mascota: Mascota, diagnostico: &str, tratamiento: &str, proxima_visita: Option<Fecha>) -> Self {
        Atencion { mascota, diagnostico: diagnostico.to_string(), tratamiento: tratamiento.to_string(), proxima_visita }
    }

    pub fn equals(&self, other: &Self) -> bool {
        let fecha_eq = match (&self.proxima_visita, &other.proxima_visita) {
            (Some(fself), Some(fother)) => fself.equals(&fother),
            (Option::None, Option::None) => true,
            _ => false
        };
        fecha_eq && self.mascota.equals(&other.mascota) && self.diagnostico == other.diagnostico && 
        self.tratamiento == other.tratamiento
    }

    pub fn equals_reducido(&self, nombre_mascota: &str, nombre_duenio: &str, telefono: &str ) -> bool {
        self.mascota.nombre.as_str() == nombre_mascota &&
        self.mascota.duenio.nombre.as_str() == nombre_duenio &&
        self.mascota.duenio.telefono.as_str() == telefono
    }
}

#[derive(Debug)]
pub struct Veterinaria {
    pub nombre: String,
    pub direccion: String,
    pub id: u64,
    pub cola_atencion: VecDeque<Mascota>,
    pub registro_atenciones: Vec<Atencion>
}

impl Veterinaria {
    pub fn new(nombre: &str, direccion: &str, id: u64) -> Self {
        Veterinaria { nombre: nombre.to_string(), direccion: direccion.to_string(), id,
            cola_atencion: VecDeque::new(), registro_atenciones: vec![] }
    }

    pub fn agregar_mascota(&mut self, mascota: Mascota) {
        self.cola_atencion.push_back(mascota);
    }

    pub fn agregar_mascota_prioritaria(&mut self, mascota: Mascota) {
        self.cola_atencion.push_front(mascota);
    }

    pub fn atender_mascota(&mut self) -> Option<Mascota> {
        self.cola_atencion.pop_front()
    }

    pub fn eliminar_mascota(&mut self, nombre_mascota: &str, nombre_duenio: &str, telefono: &str) -> bool {
        if let Some(index) = self.cola_atencion.iter().position(|x| x.equals_reducido(nombre_mascota, nombre_duenio, telefono)) {
            self.cola_atencion.remove(index);
            true
        } else {
            false
        }
    }

    pub fn registrar_atencion(&mut self, atencion: Atencion) {
        self.registro_atenciones.push(atencion);
    }

    pub fn buscar_atencion(&self, nombre_mascota: &str, nombre_duenio: &str, telefono: &str) -> Option<&Atencion> {
        for a in &self.registro_atenciones {
            if a.equals_reducido(nombre_mascota, nombre_duenio, telefono) {
                return Some(a);
            }
        }
        None
    }

    pub fn modificar_diagnostico(&mut self, nombre_mascota: &str, nombre_duenio: &str, telefono: &str, nuevo_diagnostico: &str) -> bool {
        for a in self.registro_atenciones.iter_mut() {
            if a.equals_reducido(nombre_mascota, nombre_duenio, telefono) {
                a.diagnostico = nuevo_diagnostico.to_string();
                return true;
            }
        }
        false
    }

    pub fn modificar_proxima_visita(&mut self, nombre_mascota: &str, nombre_duenio: &str, telefono: &str, nueva_fecha: Option<Fecha>) -> bool {
        for a in self.registro_atenciones.iter_mut() {
            if a.equals_reducido(nombre_mascota, nombre_duenio, telefono) {
                a.proxima_visita = nueva_fecha;
                return true;
            }
        }
        false
    }

    pub fn eliminar_atencion(&mut self, nombre_mascota: &str, nombre_duenio: &str, telefono: &str) -> bool {
        let mut index = 0;
        for a in self.registro_atenciones.iter_mut() {
            if a.equals_reducido(nombre_mascota, nombre_duenio, telefono) {
                self.registro_atenciones.remove(index);
                return true;
            }
            index += 1;
        }
        false
    }
}

#[test]
fn test_veterinaria_agregar_mascota() {
    let mut v = Veterinaria::new("Veterinaria1", "Centenario", 4839243274392);
    v.agregar_mascota(Mascota::new("Bola", 7, Animal::GATO, 
        Persona::new("Pedro", "6 y 50", "221 123-4567")));
    v.agregar_mascota(Mascota::new("Micky", 4, Animal::PERRO, 
        Persona::new("Carlos", "1 y 44", "221 891-0111")));
    assert_eq!(v.cola_atencion.len(), 2);
    assert_eq!(v.cola_atencion[0].nombre, "Bola".to_string());
    assert!(v.cola_atencion[0].tipo.equals(&Animal::GATO));
    assert_eq!(v.cola_atencion[1].nombre, "Micky".to_string());
    assert!(v.cola_atencion[1].tipo.equals(&Animal::PERRO));
}

#[test]
fn test_veterinaria_agregar_mascota_prioritaria() {
    let mut v = Veterinaria::new("Veterinaria1", "Centenario", 4839243274392);
    v.agregar_mascota_prioritaria(Mascota::new("Bola", 7, Animal::GATO, 
        Persona::new("Pedro", "6 y 50", "221 123-4567")));
    v.agregar_mascota_prioritaria(Mascota::new("Micky", 4, Animal::PERRO, 
        Persona::new("Carlos", "1 y 44", "221 891-0111")));
    assert_eq!(v.cola_atencion.len(), 2);
    assert_eq!(v.cola_atencion[0].nombre, "Micky".to_string());
    assert!(v.cola_atencion[0].tipo.equals(&Animal::PERRO));
    assert_eq!(v.cola_atencion[1].nombre, "Bola".to_string());
    assert!(v.cola_atencion[1].tipo.equals(&Animal::GATO));
}

#[test]
fn test_veterinaria_atender_mascota() {
    let mut v = Veterinaria::new("Veterinaria1", "Centenario", 4839243274392);
    v.agregar_mascota(Mascota::new("Bola", 7, Animal::GATO, 
        Persona::new("Pedro", "6 y 50", "221 123-4567")));
    v.agregar_mascota(Mascota::new("Micky", 4, Animal::PERRO, 
        Persona::new("Carlos", "1 y 44", "221 891-0111")));

    let m1 = v.atender_mascota();
    assert!(m1.is_some());
    assert_eq!(m1.unwrap().nombre, "Bola".to_string());

    let m2 = v.atender_mascota();
    assert!(m2.is_some());
    assert_eq!(m2.unwrap().nombre, "Micky".to_string());

    assert_eq!(v.cola_atencion.len(), 0);

    let mn = v.atender_mascota();
    assert!(mn.is_none());
}

#[test]
fn test_veterinaria_eliminar_mascota() {
    let mut v = Veterinaria::new("Veterinaria1", "Centenario", 4839243274392);
    v.agregar_mascota(Mascota::new("Bola", 7, Animal::GATO, 
        Persona::new("Pedro", "6 y 50", "221 123-4567")));
    v.agregar_mascota(Mascota::new("Micky", 4, Animal::PERRO, 
        Persona::new("Carlos", "1 y 44", "221 891-0111")));
    assert_eq!(v.cola_atencion.len(), 2);

    assert!(v.eliminar_mascota("Bola", "Pedro", "221 123-4567"));
    assert_eq!(v.cola_atencion.len(), 1);
    assert!(!v.eliminar_mascota("Bola", "Pedro", "221 123-4567"));
    assert_eq!(v.cola_atencion.len(), 1);
    assert!(!v.eliminar_mascota("Micky", "Pedro", "221 891-0111")); // dueño distinto
    assert_eq!(v.cola_atencion.len(), 1);
    assert!(v.eliminar_mascota("Micky", "Carlos", "221 891-0111"));
    assert_eq!(v.cola_atencion.len(), 0);
    assert!(!v.eliminar_mascota("", "", ""));
    assert_eq!(v.cola_atencion.len(), 0);
}

#[test]
fn test_veterinaria_registrar_atencion() {
    let mut v = Veterinaria::new("Veterinaria1", "Centenario", 4839243274392);
    v.registrar_atencion(Atencion::new(
        Mascota::new("Bola", 7, Animal::GATO, Persona::new("Pedro", "6 y 50", "221 123-4567")),
        "Pulgas",
        "Pipeta",
        None
    ));
    v.registrar_atencion(Atencion::new(
        Mascota::new("Micky", 4, Animal::PERRO, Persona::new("Carlos", "1 y 44", "221 891-0111")),
        "Garrapatas",
        "Antiparasitario y extraccion",
        Some(Fecha::new(12, 6, 2026))
    ));
    assert_eq!(v.registro_atenciones.len(), 2);

    assert_eq!(v.registro_atenciones[0].diagnostico, "Pulgas".to_string());
    assert!(v.registro_atenciones[0].proxima_visita.is_none());
    assert_eq!(v.registro_atenciones[1].diagnostico, "Garrapatas".to_string());
    assert!(v.registro_atenciones[1].proxima_visita.clone().unwrap().equals(&Fecha::new(12, 6, 2026)));
}

#[test]
fn test_veterinaria_buscar_atencion() {
    let mut v = Veterinaria::new("Veterinaria1", "Centenario", 4839243274392);
    v.registrar_atencion(Atencion::new(
        Mascota::new("Bola", 7, Animal::GATO, Persona::new("Pedro", "6 y 50", "221 123-4567")),
        "Pulgas",
        "Pipeta",
        None
    ));
    v.registrar_atencion(Atencion::new(
        Mascota::new("Micky", 4, Animal::PERRO, Persona::new("Carlos", "1 y 44", "221 891-0111")),
        "Garrapatas",
        "Antiparasitario y extraccion",
        Some(Fecha::new(12, 6, 2026))
    ));
    assert_eq!(v.registro_atenciones.len(), 2);

    let a1 = v.buscar_atencion("Bola", "Pedro", "221 123-4567").unwrap();
    assert_eq!(*a1.diagnostico, "Pulgas".to_string());
    assert_eq!(*a1.mascota.nombre, "Bola".to_string());

    let a1 = v.buscar_atencion("Micky", "Carlos", "221 891-0111").unwrap();
    assert_eq!(*a1.diagnostico, "Garrapatas".to_string());
    assert_eq!(*a1.mascota.nombre, "Micky".to_string());

    assert!(v.buscar_atencion("Pelos", "Maria", "221 420-6769").is_none());
    assert!(v.buscar_atencion("", "", "").is_none());
}

#[test]
fn test_veterinaria_modificar_diagnostico() {
    let mut v = Veterinaria::new("Veterinaria1", "Centenario", 4839243274392);
    v.registrar_atencion(Atencion::new(
        Mascota::new("Bola", 7, Animal::GATO, Persona::new("Pedro", "6 y 50", "221 123-4567")),
        "Pulgas",
        "Pipeta",
        None
    ));
    v.registrar_atencion(Atencion::new(
        Mascota::new("Micky", 4, Animal::PERRO, Persona::new("Carlos", "1 y 44", "221 891-0111")),
        "Garrapatas",
        "Antiparasitario y extraccion",
        Some(Fecha::new(12, 6, 2026))
    ));
    
    assert!(v.modificar_diagnostico(
        "Bola", "Pedro", "221 123-4567",
        "Pastillas"
    ));
    assert_eq!(v.registro_atenciones[0].diagnostico, "Pastillas".to_string());

    assert!(!v.modificar_diagnostico(
        "Leo", "Maria", "221 420-6769",
        "algun diagnostico"
    ));

    assert!(v.modificar_diagnostico(
        "Micky", "Carlos", "221 891-0111",
        "Garrapaticida"
    ));
    assert_eq!(v.registro_atenciones[1].diagnostico, "Garrapaticida".to_string());
}

#[test]
fn test_veterinaria_modificar_proxima_visita() {
    let mut v = Veterinaria::new("Veterinaria1", "Centenario", 4839243274392);
    v.registrar_atencion(Atencion::new(
        Mascota::new("Bola", 7, Animal::GATO, Persona::new("Pedro", "6 y 50", "221 123-4567")),
        "Pulgas",
        "Pipeta",
        None
    ));
    v.registrar_atencion(Atencion::new(
        Mascota::new("Micky", 4, Animal::PERRO, Persona::new("Carlos", "1 y 44", "221 891-0111")),
        "Garrapatas",
        "Antiparasitario y extraccion",
        Some(Fecha::new(12, 6, 2026))
    ));
    
    assert!(v.modificar_proxima_visita(
        "Bola", "Pedro", "221 123-4567",
        Some(Fecha::new(5, 7, 2026))
    ));
    assert!(v.registro_atenciones[0].proxima_visita.clone().unwrap().equals(&Fecha::new(5, 7, 2026)));

    assert!(!v.modificar_proxima_visita(
        "Leo", "Maria", "221 420-6769",
        Some(Fecha::new(1, 1, 2001))
    ));

    assert!(v.modificar_proxima_visita(
        "Micky", "Carlos", "221 891-0111",
        None
    ));
    assert!(v.registro_atenciones[1].proxima_visita.is_none());
}

#[test]
fn test_veterinaria_eliminar_atencion() {
    let mut v = Veterinaria::new("Veterinaria1", "Centenario", 4839243274392);
    v.registrar_atencion(Atencion::new(
        Mascota::new("Bola", 7, Animal::GATO, Persona::new("Pedro", "6 y 50", "221 123-4567")),
        "Pulgas",
        "Pipeta",
        None
    ));
    v.registrar_atencion(Atencion::new(
        Mascota::new("Micky", 4, Animal::PERRO, Persona::new("Carlos", "1 y 44", "221 891-0111")),
        "Garrapatas",
        "Antiparasitario y extraccion",
        Some(Fecha::new(12, 6, 2026))
    ));

    assert_eq!(v.registro_atenciones.len(), 2);

    assert!(v.eliminar_atencion("Micky", "Carlos", "221 891-0111"));
    assert_eq!(v.registro_atenciones.len(), 1);
    assert_eq!(v.registro_atenciones[0].mascota.nombre, "Bola".to_string());

    assert!(!v.eliminar_atencion("Leo", "Maria", "221 420-6769"));
    assert_eq!(v.registro_atenciones.len(), 1);

    assert!(v.eliminar_atencion("Bola", "Pedro", "221 123-4567"));
    assert_eq!(v.registro_atenciones.len(), 0);
}