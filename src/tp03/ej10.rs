#![allow(unused)]
use std::collections::HashMap;
use super::ej3::Fecha;

#[derive(Debug, Clone)]
pub enum Genero {
    NOVELA, INFANTIL, TECNICO, OTRO
}

impl Genero {
    pub fn equals(&self, other: &Self) -> bool {
        match (self, other) {
            (Genero::NOVELA, Genero::NOVELA) => true, (Genero::TECNICO, Genero::TECNICO) => true,
            (Genero::INFANTIL, Genero::INFANTIL) => true, (Genero::OTRO, Genero::OTRO) => true,
            _ => false
        }
    }
}

#[derive(Debug, Clone)]
pub enum Estado {
    Devuelto, EnPrestamo
}

impl Estado {
    pub fn equals(&self, other: &Self) -> bool {
        match (self, other) {
            (Estado::Devuelto, Estado::Devuelto) => true,
            (Estado::EnPrestamo, Estado::EnPrestamo) => true,
            _ => false
        }
    }
}

#[derive(Debug, Clone)]
pub struct Libro {
    isbn: String,
    titulo: String,
    autor: String,
    paginas: usize,
    genero: Genero
}

impl Libro {
    pub fn new(isbn: &str, titulo: &str, autor: &str, paginas: usize, genero: Genero) -> Self {
        Libro { isbn: isbn.to_string(), titulo: titulo.to_string(), autor: autor.to_string(), paginas, genero }
    }

    pub fn equals(&self, other: &Self) -> bool {
        self.isbn == other.isbn && self.titulo == other.titulo && self.autor == other.autor &&
        self.paginas == other.paginas && self.genero.equals(&other.genero)
    }
}

#[derive(Debug, Clone)]
pub struct Cliente {
    nombre: String,
    telefono: String,
    email: String
}

impl Cliente {
    pub fn new(nombre: &str, telefono: &str, email: &str) -> Self {
        Cliente { nombre: nombre.to_string(), telefono: telefono.to_string(), email: email.to_string() }
    }

    pub fn equals(&self, other: &Self) -> bool {
        self.nombre == other.nombre && self.telefono == other.telefono && self.email == other.email
    }
}

#[derive(Debug)]
pub struct Prestamo {
    libro: Libro,
    cliente: Cliente,
    fecha_vencimiento: Fecha,
    fecha_devolucion: Option<Fecha>,
    estado: Estado
}

impl Prestamo {
    pub fn new(libro: Libro, cliente: Cliente, fecha_vencimiento: Fecha, fecha_devolucion: Option<Fecha>, estado: Estado) -> Self {
        Prestamo { libro, cliente, fecha_devolucion, fecha_vencimiento, estado }
    }
}

#[derive(Debug)]
pub struct Biblioteca {
    nombre: String,
    direccion: String,
    libros_disponibles: HashMap<String, usize>,
    prestamos_efectuados: Vec<Prestamo>
}

impl Biblioteca {
    pub fn new(nombre: &str, direccion: &str) -> Self {
        Biblioteca { nombre: nombre.to_string(), direccion: direccion.to_string(), libros_disponibles: HashMap::new(), prestamos_efectuados: vec![] }
    }

    pub fn cantidad_copias(&self, libro: &Libro) -> usize {
        if let Some(cant) = self.libros_disponibles.get(&libro.isbn) {
            return *cant
        }
        0
    }

    pub fn decrementar_copias(&mut self, libro: &Libro) -> bool {
        if let Some(cant) = self.libros_disponibles.get_mut(&libro.isbn) {
            if *cant > 0 {
                *cant -= 1;
                return true
            }
        }
        false
    }

    pub fn incrementar_copias(&mut self, libro: &Libro) {
        if let Some(cant) = self.libros_disponibles.get_mut(&libro.isbn) {
            *cant += 1;
        } else {
            self.libros_disponibles.insert(libro.isbn.clone(), 1);
        }
    }

    pub fn contar_prestamos(&self, email_cliente: &str) -> usize {
        let mut cant = 0_usize;
        for prestamo in &self.prestamos_efectuados {
            if prestamo.cliente.email.as_str() == email_cliente && prestamo.estado.equals(&Estado::EnPrestamo) {
                cant += 1;
            }
        }
        cant
    }

    pub fn realizar_prestamo(&mut self, cliente: Cliente, libro: Libro, fecha_vencimiento: Fecha) -> bool {
        if self.contar_prestamos(&cliente.email) > 5 {
            return false
        }

        if (!self.decrementar_copias(&libro)) {
            return false
        }

        self.prestamos_efectuados.push(Prestamo::new(
            libro, cliente, fecha_vencimiento, None, Estado::EnPrestamo
        ));
        true
    }

    pub fn prestamos_a_vencer(&self, antes_de_dias: u32) -> Vec<&Prestamo> {
        let mut prest_vencer = vec![];
        let mut fecha_limite = Fecha::fecha_actual(); fecha_limite.sumar_dias(antes_de_dias);
        for prestamo in &self.prestamos_efectuados {
            if prestamo.estado.equals(&Estado::EnPrestamo) && fecha_limite.es_mayor(&prestamo.fecha_vencimiento) {
                prest_vencer.push(prestamo);
            }
        }
        prest_vencer
    }

    pub fn prestamos_vencidos(&self) -> Vec<&Prestamo> {
        self.prestamos_a_vencer(0)
    }

    pub fn buscar_prestamo(&mut self, libro: &Libro, cliente: &Cliente) -> Option<&mut Prestamo> {
        for prestamo in self.prestamos_efectuados.iter_mut() {
            if prestamo.libro.equals(libro) && prestamo.cliente.equals(cliente) {
                return Some(prestamo)
            }
        }
        None
    }

    pub fn devolver_libro(&mut self, libro: &Libro, cliente: &Cliente, fecha_devolucion: Fecha) -> bool {
        if let Some(prestamo) = self.buscar_prestamo(libro, cliente) {
            if prestamo.estado.equals(&Estado::EnPrestamo) {
                prestamo.estado = Estado::Devuelto;
                prestamo.fecha_devolucion = Some(fecha_devolucion);
                self.incrementar_copias(libro);
                return true
            }
        }
        false
    }
}

#[test]
fn test_biblioteca_cantidad_copias() {
    let mut b = Biblioteca::new("Biblioteca La Plata", "Plaza Dardo Rocha 137");
    let l1 = Libro::new(
        "978-0-306-40615-7", "Harry Potter y la Piedra Filosofal", 
        "J. K. Rowling", 260, Genero::NOVELA
    );
    assert_eq!(b.cantidad_copias(&l1), 0);
    b.incrementar_copias(&l1);
    assert_eq!(b.cantidad_copias(&l1), 1);
}

#[test]
fn test_biblioteca_incrementar_copias() {
    let mut b = Biblioteca::new("Biblioteca La Plata", "Plaza Dardo Rocha 137");
    let l1 = Libro::new(
        "978-8-478-88445-2" , "Harry Potter y la Piedra Filosofal", 
        "J. K. Rowling", 260, Genero::NOVELA
    );
    let l2 = Libro::new(
        "978-8-427-20212-2", "Los Juegos del Hambre", 
        "Suzanne Collins", 396, Genero::OTRO
    );
    b.incrementar_copias(&l1);
    assert_eq!(b.cantidad_copias(&l1), 1);
    b.incrementar_copias(&l1);
    assert_eq!(b.cantidad_copias(&l1), 2);
    b.incrementar_copias(&l2);
    assert_eq!(b.cantidad_copias(&l2), 1);
}

#[test]
fn test_biblioteca_decrementar_copias() {
    let mut b = Biblioteca::new("Biblioteca La Plata", "Plaza Dardo Rocha 137");
    let l1 = Libro::new(
        "978-8-478-88445-2" , "Harry Potter y la Piedra Filosofal", 
        "J. K. Rowling", 260, Genero::NOVELA
    );
    let l2 = Libro::new(
        "978-8-427-20212-2", "Los Juegos del Hambre", 
        "Suzanne Collins", 396, Genero::OTRO
    );
    assert!(!b.decrementar_copias(&l1));
    assert!(!b.decrementar_copias(&l2));

    b.incrementar_copias(&l1);
    b.incrementar_copias(&l1);

    b.incrementar_copias(&l2);
    b.incrementar_copias(&l2);
    b.incrementar_copias(&l2);

    assert!(b.decrementar_copias(&l1));
    assert_eq!(b.cantidad_copias(&l1), 1);
    assert!(b.decrementar_copias(&l1));
    assert_eq!(b.cantidad_copias(&l1), 0);
    assert!(!b.decrementar_copias(&l1));

    assert!(b.decrementar_copias(&l2));
    assert_eq!(b.cantidad_copias(&l2), 2);
    assert!(b.decrementar_copias(&l2));
    assert_eq!(b.cantidad_copias(&l2), 1);
    assert!(b.decrementar_copias(&l2));
    assert_eq!(b.cantidad_copias(&l2), 0);
    assert!(!b.decrementar_copias(&l2));
}

#[test]
fn test_biblioteca_contar_prestamos() {
    let mut b = Biblioteca::new("Biblioteca La Plata", "Plaza Dardo Rocha 137");
    let l1 = Libro::new(
        "978-8-478-88445-2" , "Harry Potter y la Piedra Filosofal", 
        "J. K. Rowling", 260, Genero::NOVELA
    );
    let l2 = Libro::new(
        "978-8-427-20212-2", "Los Juegos del Hambre", 
        "Suzanne Collins", 396, Genero::OTRO
    );
    b.incrementar_copias(&l1); b.incrementar_copias(&l1); b.incrementar_copias(&l1);
    

    let cliente = Cliente::new("Pedro", "221 123-4567", "pedro@gmail.com");
    assert_eq!(b.contar_prestamos("pedro@gmail.com"), 0);
    assert!(b.realizar_prestamo(cliente.clone(), l1.clone(), Fecha::new(16, 6, 2026)));
    assert_eq!(b.contar_prestamos("pedro@gmail.com"), 1);
    assert!(b.realizar_prestamo(cliente.clone(), l1.clone(), Fecha::new(16, 6, 2026)));
    assert_eq!(b.contar_prestamos("pedro@gmail.com"), 2);

    assert_eq!(b.contar_prestamos("maria@hotmail.com"), 0);

    assert!(b.devolver_libro(&l1, &cliente, Fecha::new(19, 5, 2026)));
    assert_eq!(b.contar_prestamos(cliente.email.as_str()), 1);
}

#[test]
fn test_biblioteca_realizar_prestamo() {
    let mut b = Biblioteca::new("Biblioteca La Plata", "Plaza Dardo Rocha 137");
    let l1 = Libro::new(
        "978-8-478-88445-2" , "Harry Potter y la Piedra Filosofal", 
        "J. K. Rowling", 260, Genero::NOVELA
    );
    let l2 = Libro::new(
        "978-8-427-20212-2", "Los Juegos del Hambre", 
        "Suzanne Collins", 396, Genero::OTRO
    );
    for i in 0..5 { // 5 copias de cada libro
        b.incrementar_copias(&l1);
        b.incrementar_copias(&l2);
    }
    

    let cliente = Cliente::new("Pedro", "221 123-4567", "pedro@gmail.com");
    assert!(!b.realizar_prestamo(cliente.clone(), Libro::new("1234", "libro", "autor", 1000, Genero::OTRO), Fecha::new(15, 7, 2026))); // libro no existe
    assert!(b.realizar_prestamo(cliente.clone(), l1.clone(), Fecha::new(16, 6, 2026)));
    assert!(b.realizar_prestamo(cliente.clone(), l1.clone(), Fecha::new(18, 6, 2026)));
    assert!(b.realizar_prestamo(cliente.clone(), l2.clone(), Fecha::new(5, 6, 2026)));
    assert!(b.realizar_prestamo(cliente.clone(), l2.clone(), Fecha::new(24, 5, 2026)));
    assert!(b.realizar_prestamo(cliente.clone(), l1.clone(), Fecha::new(29, 7, 2026)));
    assert!(b.realizar_prestamo(cliente.clone(), l2.clone(), Fecha::new(4, 8, 2026)));
    assert!(!b.realizar_prestamo(cliente.clone(), l2.clone(), Fecha::new(15, 7, 2026))); // mas de 5 prestamos "en prestamo" alcanzados

    b.devolver_libro(&l1, &cliente, Fecha::new(19, 5, 2026));
    assert!(b.realizar_prestamo(cliente.clone(), l1.clone(), Fecha::new(15, 7, 2026))); // mas de 5 prestamos alcanzados pero 5 "en prestamo", 1 "devuelto"
}

#[test]
fn test_biblioteca_prestamos_a_vencer() {
    let mut b = Biblioteca::new("Biblioteca La Plata", "Plaza Dardo Rocha 137");
    let l1 = Libro::new(
        "978-8-478-88445-2" , "Harry Potter y la Piedra Filosofal", 
        "J. K. Rowling", 260, Genero::NOVELA
    );
    let l2 = Libro::new(
        "978-8-427-20212-2", "Los Juegos del Hambre", 
        "Suzanne Collins", 396, Genero::OTRO
    );
    for i in 0..5 {
        b.incrementar_copias(&l1);
        b.incrementar_copias(&l2);
    }
    
    // fecha actual es 17/5/2026 (para testear)
    assert!(Fecha::fecha_actual().equals_fecha(17, 5, 2026));

    let cliente = Cliente::new("Pedro", "221 123-4567", "pedro@gmail.com");
    assert!(b.realizar_prestamo(cliente.clone(), l2.clone(), Fecha::new(24, 5, 2026))); // vence en 8 dias (dia siguiente al 24)
    assert!(b.realizar_prestamo(cliente.clone(), l2.clone(), Fecha::new( 5, 6, 2026))); // vence en 20 dias
    assert!(b.realizar_prestamo(cliente.clone(), l1.clone(), Fecha::new(16, 6, 2026))); // vence en 31 dias
    assert!(b.realizar_prestamo(cliente.clone(), l1.clone(), Fecha::new(18, 6, 2026))); // vence en 33 dias
    assert!(b.realizar_prestamo(cliente.clone(), l1.clone(), Fecha::new(29, 7, 2026))); // vence en 74 dias
    assert!(b.realizar_prestamo(cliente.clone(), l2.clone(), Fecha::new( 4, 8, 2026))); // vence en 80 dias

    assert_eq!(b.prestamos_a_vencer(15).len(), 1); // en 15 dias -> 1/6/2026
    assert_eq!(b.prestamos_a_vencer(30).len(), 2); // en 30 dias -> 16/6/2026
    assert_eq!(b.prestamos_a_vencer(31).len(), 3); // en 31 dias -> 17/6/2026
    assert_eq!(b.prestamos_a_vencer(74).len(), 5); // en 60 dias -> 30/7/2026
    assert_eq!(b.prestamos_a_vencer(100).len(), 6); // en 100 dias -> 25/8/2026
}

#[test]
fn test_biblioteca_prestamos_vencidos() {
    let mut b = Biblioteca::new("Biblioteca La Plata", "Plaza Dardo Rocha 137");
    let l1 = Libro::new(
        "978-8-478-88445-2" , "Harry Potter y la Piedra Filosofal", 
        "J. K. Rowling", 260, Genero::NOVELA
    );
    let l2 = Libro::new(
        "978-8-427-20212-2", "Los Juegos del Hambre", 
        "Suzanne Collins", 396, Genero::OTRO
    );
    for i in 0..5 {
        b.incrementar_copias(&l1);
        b.incrementar_copias(&l2);
    }
    
    // fecha actual es 17/5/2026 (para testear)
    assert!(Fecha::fecha_actual().equals_fecha(17, 5, 2026));

    let cliente = Cliente::new("Pedro", "221 123-4567", "pedro@gmail.com");
    assert!(b.realizar_prestamo(cliente.clone(), l2.clone(), Fecha::new(10, 4, 2026))); // vencido
    assert!(b.realizar_prestamo(cliente.clone(), l2.clone(), Fecha::new(28, 4, 2026))); // vencido 
    assert!(b.realizar_prestamo(cliente.clone(), l1.clone(), Fecha::new(13, 5, 2026))); // vencido
    assert!(b.realizar_prestamo(cliente.clone(), l1.clone(), Fecha::new(18, 5, 2026))); // no vencido
    assert!(b.realizar_prestamo(cliente.clone(), l1.clone(), Fecha::new( 5, 6, 2026))); // no vencido
    assert!(b.realizar_prestamo(cliente.clone(), l2.clone(), Fecha::new(15, 7, 2026))); // no vencido

    assert_eq!(b.prestamos_vencidos().len(), 3);
}

#[test]
fn test_biblioteca_buscar_prestamo() {
    let mut b = Biblioteca::new("Biblioteca La Plata", "Plaza Dardo Rocha 137");
    let l1 = Libro::new(
        "978-8-478-88445-2" , "Harry Potter y la Piedra Filosofal", 
        "J. K. Rowling", 260, Genero::NOVELA
    );
    let l2 = Libro::new(
        "978-8-427-20212-2", "Los Juegos del Hambre", 
        "Suzanne Collins", 396, Genero::OTRO
    );
    for i in 0..5 {
        b.incrementar_copias(&l1);
        b.incrementar_copias(&l2);
    }

    let cliente = Cliente::new("Pedro", "221 123-4567", "pedro@gmail.com");
    assert!(b.realizar_prestamo(cliente.clone(), l2.clone(), Fecha::new(10, 6, 2026)));
    assert!(b.realizar_prestamo(cliente.clone(), l1.clone(), Fecha::new(13, 7, 2026)));

    assert!(b.buscar_prestamo(&l1, &cliente).unwrap().cliente.equals(&cliente));
    assert!(b.buscar_prestamo(
        &Libro::new(
            "978-8-427-20212-2", "Los Juegos del Hambre", 
            "Suzanne Collins", 396, Genero::OTRO
        ),
        &Cliente::new("Pedro", "221 123-4567", "pedro@gmail.com")
    ).unwrap().estado.equals(&Estado::EnPrestamo));

    assert!(b.devolver_libro(&l1, &cliente, Fecha::new(20, 5, 2026)));
    assert!(b.buscar_prestamo(&l1, &cliente).unwrap().estado.equals(&Estado::Devuelto));

    assert!(b.buscar_prestamo(&l1, &Cliente::new("abc", "def", "ghi@jkl.mno")).is_none()); // prestamo que no existe
    let mut l2 = l1.clone();
    l2.genero = Genero::INFANTIL;
    assert!(b.buscar_prestamo(&l2, &cliente).is_none()); // libro de distinto genero
}

#[test]
fn test_biblioteca_devolver_libro() {
    let mut b = Biblioteca::new("Biblioteca La Plata", "Plaza Dardo Rocha 137");
    let l1 = Libro::new(
        "978-8-478-88445-2" , "Harry Potter y la Piedra Filosofal", 
        "J. K. Rowling", 260, Genero::NOVELA
    );
    let l2 = Libro::new(
        "978-8-427-20212-2", "Los Juegos del Hambre", 
        "Suzanne Collins", 396, Genero::OTRO
    );
    for i in 0..5 {
        b.incrementar_copias(&l1);
        b.incrementar_copias(&l2);
    }

    let cliente = Cliente::new("Pedro", "221 123-4567", "pedro@gmail.com");
    assert!(b.realizar_prestamo(cliente.clone(), l2.clone(), Fecha::new(10, 6, 2026)));
    assert!(b.realizar_prestamo(cliente.clone(), l1.clone(), Fecha::new(13, 7, 2026)));

    assert!(b.devolver_libro(&l1, &cliente, Fecha::new(20, 5, 2026)));
    assert!(!b.devolver_libro(&l1, &cliente, Fecha::new(20, 5, 2026))); // libro ya devuelto, no se puede volver a devolver
    assert!(b.devolver_libro(&l2, &cliente, Fecha::new(20, 5, 2026)));
}