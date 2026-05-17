#![allow(unused)]
pub struct Examen {
    pub materia: String,
    pub nota: f32
}

impl Examen {
    pub fn new(materia: &str, nota: f32) -> Self {
        assert!(nota >= 0.0 && nota <= 10.0, "La nota debe estar entre 0 y 10");
        Examen { materia: materia.to_string(), nota }
    }
}

pub struct Estudiante {
    pub nombre: String,
    pub id: u64,
    calificaciones: Vec<Examen>
}

impl Estudiante {
    pub fn new(nombre: &str, id: u64, calificaciones: Vec<Examen>) -> Self {
        Estudiante { nombre: nombre.to_string(), id, calificaciones }
    }

    pub fn obtener_promedio(&self) -> f32 {
        assert!(self.calificaciones.len() > 0, "El estudiante no tiene examenes registrados");
        let mut suma = 0.0;
        for ex in &self.calificaciones {
            suma += ex.nota;
        }
        suma / self.calificaciones.len() as f32
    }

    pub fn obtener_calificacion_mas_alta(&self) -> f32 {
        assert!(self.calificaciones.len() > 0, "El estudiante no tiene examenes registrados");
        let mut max = 0.0;
        for ex in &self.calificaciones {
            if ex.nota > max {
                max = ex.nota;
            }
        }
        max
    }

    pub fn obtener_calificacion_mas_baja(&self) -> f32 {
        assert!(self.calificaciones.len() > 0, "El estudiante no tiene examenes registrados");
        let mut min = f32::MAX;
        for ex in &self.calificaciones {
            if ex.nota < min{
                min = ex.nota;
            }
        }
        min
    }
}

#[test]
fn test_examen_creacion_valido() {
    Examen::new("Seminario de lenguajes", 5.0);
}

#[should_panic(expected="La nota debe estar entre 0 y 10")]
#[test]
fn test_examen_creacion_invalido_menor_0() {
    Examen::new("Seminario de lenguajes", -1.0);
}

#[should_panic(expected="La nota debe estar entre 0 y 10")]
#[test]
fn test_examen_creacion_invalido_mayor_10() {
    Examen::new("Seminario de lenguajes", 11.0);
}

#[test]
fn test_estudiante_promedio() {
    let e = Estudiante::new("Juan", 1234, 
    vec![Examen::new("mate1", 7.0), Examen::new("mate2", 8.5)]);
    assert_eq!(e.obtener_promedio(), 7.75);

    let e = Estudiante::new("Maria", 91011, 
    vec![Examen::new("mate1", 0.0), Examen::new("mate2", 0.0), Examen::new("mate3", 0.0)]);
    assert_eq!(e.obtener_promedio(), 0.0);
}

#[should_panic(expected="El estudiante no tiene examenes registrados")]
#[test]
fn test_estudiante_promedio_0_examenes() {
    let e = Estudiante::new("Pedro", 5678, 
    vec![]);
    e.obtener_promedio();
}

#[test]
fn test_estudiante_calificacion_mas_alta() {
    let e = Estudiante::new("Juan", 1234, 
    vec![Examen::new("mate1", 7.0), Examen::new("mate2", 8.5)]);
    let max = e.obtener_calificacion_mas_alta();
    assert_eq!(max, 8.5);

    let e = Estudiante::new("Maria", 91011, 
    vec![Examen::new("mate1", 0.0), Examen::new("mate2", 0.0), Examen::new("mate3", 0.0)]);
    let max = e.obtener_calificacion_mas_alta();
    assert_eq!(max, 0.0);

    let e = Estudiante::new("Garcia", 1235813, 
    vec![Examen::new("mate1", 6.98), Examen::new("mate1", 8.6), Examen::new("mate2", 5.66), Examen::new("mate3", 8.98), Examen::new("mate3", 9.9)]);
    let max = e.obtener_calificacion_mas_alta();
    assert_eq!(max, 9.9);
}

#[should_panic(expected="El estudiante no tiene examenes registrados")]
#[test]
fn test_estudiante_calificacion_mas_alta_0_examenes() {
    let e = Estudiante::new("Pedro", 5678, 
    vec![]);
    e.obtener_calificacion_mas_alta();
}

#[test]
fn test_estudiante_calificacion_mas_baja() {
    let e = Estudiante::new("Juan", 1234, 
    vec![Examen::new("mate1", 7.0), Examen::new("mate2", 8.5)]);
    let min = e.obtener_calificacion_mas_baja();
    assert_eq!(min, 7.0);

    let e = Estudiante::new("Maria", 91011, 
    vec![Examen::new("mate1", 0.0), Examen::new("mate2", 0.0), Examen::new("mate3", 0.0)]);
    let min = e.obtener_calificacion_mas_baja();
    assert_eq!(min, 0.0);

    let e = Estudiante::new("Garcia", 1235813, 
    vec![Examen::new("mate1", 6.98), Examen::new("mate1", 8.6), Examen::new("mate2", 5.66), Examen::new("mate3", 8.98), Examen::new("mate3", 9.9)]);
    let min = e.obtener_calificacion_mas_baja();
    assert_eq!(min, 5.66);
}

#[should_panic(expected="El estudiante no tiene examenes registrados")]
#[test]
fn test_estudiante_calificacion_mas_baja_0_examenes() {
    let e = Estudiante::new("Pedro", 5678, 
    vec![]);
    e.obtener_calificacion_mas_baja();
}