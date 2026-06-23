#![allow(unused)]
use std::cmp::Ordering;

#[derive(PartialEq, Debug, Clone)]
pub struct Persona<'a> {
    nombre: &'a str,
    apellido: &'a str,
    direccion: &'a str,
    ciudad: &'a str,
    salario: f64,
    edad: u8
}

impl<'a> Persona<'a> {
    pub fn new(nombre: &'a str, apellido: &'a str, direccion: &'a str, ciudad: &'a str, salario: f64, edad: u8) -> Self {
        Persona { nombre, apellido, direccion, ciudad, salario, edad }
    }

    pub fn get_salario(&self) -> f64 {
        self.salario
    }

    pub fn get_edad(&self) -> u8 {
        self.edad
    }

    pub fn get_ciudad(&self) -> &str {
        self.ciudad
    }
}

pub fn salario_mayor<'a>(vp: &'a Vec<Persona<'a>>, salario: f64) -> Vec<&'a Persona<'a>> {
    vp.iter().filter(|p| p.get_salario() > salario).collect::<Vec<_>>()
}

pub fn edad_mayor_y_vive_ciudad<'a>(vp: &'a Vec<Persona<'a>>, edad: u8, ciudad: &'a str) -> Vec<&'a Persona<'a>> {
    vp.iter().filter(|p| p.get_ciudad() == ciudad && p.get_edad() > edad).collect::<Vec<_>>()
}

pub fn todos_viven_en_ciudad<'a>(vp: &'a Vec<Persona<'a>>, ciudad: &'a str) -> bool {
    vp.iter().all(|p| p.get_ciudad() == ciudad)
}

pub fn alguno_vive_en_ciudad<'a>(vp: &'a Vec<Persona<'a>>, ciudad: &'a str) -> bool {
    vp.iter().any(|p| p.get_ciudad() == ciudad)
}

pub fn existe_persona<'a>(ap: &'a[Persona<'a>], persona: &'a Persona<'a>) -> bool {
    ap.contains(persona)
    //ap.iter().any(|p| p == persona)
}

pub fn obtener_edades<'a, const N: usize>(ap: &[Persona<'a>; N]) -> [u8; N] {
    ap.iter().map(|p| p.get_edad()).collect::<Vec<u8>>().try_into().expect("Error convirtiendo a arreglo")
}

pub fn menor_mayor_salario<'a>(ap: &'a [Persona<'a>]) -> (Option<&'a Persona<'a>>, Option<&'a Persona<'a>>) {
    let min = ap.iter().min_by(|p1, p2| {
        match p1.get_salario().total_cmp(&p2.get_salario()) {
            Ordering::Equal => p2.get_edad().cmp(&p1.get_edad()),
            ord => ord
        }
    });

    let max = ap.iter().max_by(|p1, p2| {
        match p1.get_salario().total_cmp(&p2.get_salario()) {
            Ordering::Equal => p1.get_edad().cmp(&p2.get_edad()),
            ord => ord
        }
    });
    (min, max)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_salario_mayor() {
        let vp = vec![
            Persona::new("Pedro", "Perez", "7 y 50", "La Plata", 200000.0, 22),
            Persona::new("Maria", "Sanchez", "6 y 498", "Gonnet", 125000.0, 25),
            Persona::new("Juan", "Perez", "9 y 510", "Ringuelet", 230000.0, 23),
            Persona::new("Jorge", "Toledo", "2 y 47", "La Plata", 95000.0, 19),
            Persona::new("Liam", "Chelsea", "12 y 495", "Gonnet", 300000.0, 27)
        ];
        assert_eq!(salario_mayor(&vp, 150000.0).len(), 3);
        assert_eq!(salario_mayor(&vp, 90000.0).len(), 5);
        assert_eq!(salario_mayor(&vp, 275000.0).len(), 1);
    }

    #[test]
    fn test_edad_mayor_y_vive_ciudad() {
        let vp = vec![
            Persona::new("Pedro", "Perez", "7 y 50", "La Plata", 200000.0, 22),
            Persona::new("Maria", "Sanchez", "6 y 498", "Gonnet", 125000.0, 25),
            Persona::new("Juan", "Perez", "9 y 510", "Ringuelet", 230000.0, 23),
            Persona::new("Jorge", "Toledo", "2 y 47", "La Plata", 95000.0, 19),
            Persona::new("Liam", "Chelsea", "12 y 495", "Gonnet", 300000.0, 27)
        ];
        assert_eq!(edad_mayor_y_vive_ciudad(&vp, 20, "La Plata").len(), 1);
        assert_eq!(edad_mayor_y_vive_ciudad(&vp, 28, "Gonnet").len(), 0);
        assert_eq!(edad_mayor_y_vive_ciudad(&vp, 18, "Gonnet").len(), 2);
    }

    #[test]
    fn test_todos_viven_en_ciudad() {
        let vp = vec![
            Persona::new("Pedro", "Perez", "7 y 50", "La Plata", 200000.0, 22),
            Persona::new("Maria", "Sanchez", "6 y 498", "Gonnet", 125000.0, 25),
            Persona::new("Juan", "Perez", "9 y 510", "Ringuelet", 230000.0, 23),
            Persona::new("Jorge", "Toledo", "2 y 47", "La Plata", 95000.0, 19),
            Persona::new("Liam", "Chelsea", "12 y 495", "Gonnet", 300000.0, 27)
        ];
        assert!(!todos_viven_en_ciudad(&vp, "La Plata"));
        assert!(!todos_viven_en_ciudad(&vp, "Gonnet"));

        let vp = vec![
            Persona::new("Pedro", "Perez", "7 y 50", "Gonnet", 200000.0, 22),
            Persona::new("Maria", "Sanchez", "6 y 498", "Gonnet", 125000.0, 25),
            Persona::new("Juan", "Perez", "9 y 510", "Gonnet", 230000.0, 23),
            Persona::new("Jorge", "Toledo", "2 y 47", "Gonnet", 95000.0, 19),
            Persona::new("Liam", "Chelsea", "12 y 495", "Gonnet", 300000.0, 27)
        ];
        assert!(!todos_viven_en_ciudad(&vp, "La Plata"));
        assert!(todos_viven_en_ciudad(&vp, "Gonnet"));
    }

    #[test]
    fn test_alguno_vive_en_ciudad() {
        let vp = vec![
            Persona::new("Pedro", "Perez", "7 y 50", "La Plata", 200000.0, 22),
            Persona::new("Maria", "Sanchez", "6 y 498", "Gonnet", 125000.0, 25),
            Persona::new("Juan", "Perez", "9 y 510", "Ringuelet", 230000.0, 23),
            Persona::new("Jorge", "Toledo", "2 y 47", "La Plata", 95000.0, 19),
            Persona::new("Liam", "Chelsea", "12 y 495", "Gonnet", 300000.0, 27)
        ];
        assert!(alguno_vive_en_ciudad(&vp, "La Plata"));
        assert!(alguno_vive_en_ciudad(&vp, "Gonnet"));
        assert!(!alguno_vive_en_ciudad(&vp, "Tolosa"));

        let vp = vec![
            Persona::new("Pedro", "Perez", "7 y 50", "Gonnet", 200000.0, 22),
            Persona::new("Maria", "Sanchez", "6 y 498", "Gonnet", 125000.0, 25),
            Persona::new("Juan", "Perez", "9 y 510", "Gonnet", 230000.0, 23),
            Persona::new("Jorge", "Toledo", "2 y 47", "Gonnet", 95000.0, 19),
            Persona::new("Liam", "Chelsea", "12 y 495", "Gonnet", 300000.0, 27)
        ];
        assert!(!alguno_vive_en_ciudad(&vp, "La Plata"));
        assert!(alguno_vive_en_ciudad(&vp, "Gonnet"));
    }

    #[test]
    fn test_existe_persona() {
        let p1 = Persona::new("Pedro", "Perez", "7 y 50", "La Plata", 200000.0, 22);
        let p2 = Persona::new("Maria", "Sanchez", "6 y 498", "Gonnet", 125000.0, 25);
        let p3 = Persona::new("Juan", "Perez", "9 y 510", "Ringuelet", 230000.0, 23);
        let ap = [
            p1.clone(), p2.clone(),
            Persona::new("Jorge", "Toledo", "2 y 47", "La Plata", 95000.0, 19),
            Persona::new("Liam", "Chelsea", "12 y 495", "Gonnet", 300000.0, 27)
        ];
        assert!(existe_persona(&ap, &p1));
        assert!(existe_persona(&ap, &p2));
        assert!(!existe_persona(&ap, &p3));
    }

    #[test]
    fn test_obtener_edades() {
        let ap = [
            Persona::new("Pedro", "Perez", "7 y 50", "La Plata", 200000.0, 22),
            Persona::new("Maria", "Sanchez", "6 y 498", "Gonnet", 125000.0, 25),
            Persona::new("Juan", "Perez", "9 y 510", "Ringuelet", 230000.0, 23),
            Persona::new("Jorge", "Toledo", "2 y 47", "La Plata", 95000.0, 19),
            Persona::new("Liam", "Chelsea", "12 y 495", "Gonnet", 300000.0, 27)
        ];
        assert_eq!(obtener_edades(&ap), [22, 25, 23, 19, 27]);
        assert_eq!(obtener_edades(&[]), []);
    }

    #[test]
    fn test_menor_mayor_salario() {
        let pmenor = Persona::new("Jorge", "Toledo", "2 y 47", "La Plata", 95000.0, 22);
        let pmayor = Persona::new("Liam", "Chelsea", "12 y 495", "Gonnet", 300000.0, 27);
        let ap = [
            Persona::new("Pedro", "Perez", "7 y 50", "La Plata", 95000.0, 19), // mismo salario que el menor pero menor edad
            pmenor.clone(),
            Persona::new("Maria", "Sanchez", "6 y 498", "Gonnet", 300000.0, 25), // mismo salario que el mayor pero menor edad
            pmayor.clone(),
            Persona::new("Juan", "Perez", "9 y 510", "Ringuelet", 230000.0, 23)
        ];
        assert_eq!(menor_mayor_salario(&ap), (Some(&pmenor), Some(&pmayor)));

        assert_eq!(menor_mayor_salario(&[]), (None, None));

        let p = Persona::new("Jorge", "Toledo", "2 y 47", "La Plata", 95000.0, 19);
        assert_eq!(menor_mayor_salario(std::slice::from_ref(&p)), (Some(&p), Some(&p)));

        let p1 = Persona::new("Jorge", "Toledo", "2 y 47", "La Plata", 195000.0, 19);
        let p2 = Persona::new("Jorge", "Toledo", "2 y 47", "La Plata", 195000.0, 22);
        assert_eq!(menor_mayor_salario(&[p1.clone(), p2.clone()]), (Some(&p2), Some(&p2)));
    }
}