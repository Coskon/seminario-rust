#![allow(unused)]
#[derive(PartialEq, Debug, Clone)]
pub enum Genero {
    ROCK, POP, RAP, JAZZ, OTROS
}

#[derive(PartialEq, Debug, Clone)]
pub struct Cancion {
    pub titulo: String,
    pub artista: String,
    pub genero: Genero
}

impl Cancion {
    pub fn new(titulo: &str, artista: &str, genero: Genero) -> Self {
        Cancion { titulo: titulo.to_string(), artista: artista.to_string(), genero }
    }
}

pub struct Playlist {
    pub nombre: String,
    canciones: Vec<Cancion>
}

impl Playlist {
    pub fn new(nombre: &str, canciones: Vec<Cancion>) -> Self {
        Playlist { nombre: nombre.to_string(), canciones }
    }

    pub fn agregar_cancion(&mut self, cancion: Cancion) {
        self.canciones.push(cancion);
    }

    pub fn eliminar_cancion(&mut self, nombre: &str) -> bool {
        if let Some(cancion) = self.buscar_cancion(nombre) {
            self.canciones.remove(self.canciones.iter().position(|x| *x == cancion).unwrap());
            true
        } else {
            false
        }
    }

    pub fn mover_cancion(&mut self, nombre: &str, index: usize) -> bool {
        if index >= self.canciones.len() {
            return false
        }
        if let Some(cancion) = self.buscar_cancion(nombre) {
            let prev = self.canciones.iter().position(|x| *x == cancion).unwrap();
            if prev == index {
                return true
            }
            self.canciones.remove(prev);
            self.canciones.insert(index, cancion);
            true
        } else {
            false
        }
    }

    pub fn buscar_cancion(&self, nombre: &str) -> Option<Cancion> {
        let nom = nombre.to_lowercase().to_string();
        for cancion in &self.canciones {
            if cancion.titulo.to_lowercase() == nom {
                return Some(cancion.clone());
            }
        }
        None
    }

    pub fn canciones_de_genero(&self, genero: Genero) -> Vec<Cancion> {
        let mut lista = vec![];
        for cancion in &self.canciones {
            if cancion.genero == genero {
                lista.push(cancion.clone());
            }
        }
        lista
    }

    pub fn canciones_de_artista(&self, artista: &str) -> Vec<Cancion> {
        let mut lista = vec![];
        let art = artista.to_lowercase();
        for cancion in &self.canciones {
            if cancion.artista.as_str().to_lowercase() == art {
                lista.push(cancion.clone());
            }
        }
        lista
    }

    pub fn modificar_titulo(&mut self, titulo: &str) {
        self.nombre = titulo.to_string();
    }

    pub fn borrar_canciones(&mut self) {
        self.canciones.clear();
    }

    pub fn len(&self) -> usize {
        self.canciones.len()
    }
}


#[test]
fn test_playlist_agregar_cancion() {
    let mut p = Playlist::new("playlist1", vec![]);
    assert_eq!(p.canciones.len(), 0);
    p.agregar_cancion(Cancion::new("Smooth Criminal", "Michael Jackson", Genero::POP));
    assert_eq!(p.canciones.len(), 1);
    p.agregar_cancion(Cancion::new("", "", Genero::OTROS));
    assert_eq!(p.canciones.len(), 2);
    p.agregar_cancion(Cancion::new("The Show Must Go On", "Queen", Genero::ROCK));
    assert_eq!(p.canciones.len(), 3);
    assert_eq!(p.canciones[2].titulo, "The Show Must Go On".to_string());
}

#[test]
fn test_playlist_eliminar_cancion() {
    let mut p = Playlist::new("playlist1", vec![
        Cancion::new("Smooth Criminal", "Michael Jackson", Genero::POP),
        Cancion::new("Nothing else Matters", "Metallica", Genero::OTROS),
        Cancion::new("Nothing else Matters", "Metallica", Genero::OTROS),
        Cancion::new("The Show Must Go On", "Queen", Genero::ROCK)
    ]);
    assert!(p.eliminar_cancion("Smooth Criminal"));
    assert!(!p.eliminar_cancion("Smooth Criminal"));
    assert!(!p.eliminar_cancion("Beat It"));
    assert_eq!(p.canciones.len(), 3);
    assert!(p.eliminar_cancion("nothing else matters")); 
    assert_eq!(p.canciones.len(), 2); // borrar cancion repetida solo borra 1
    // checkear que preserva el orden (importante en playlist)
    assert_eq!(p.canciones[0].titulo, "Nothing else Matters".to_string());
    assert_eq!(p.canciones[1].titulo, "The Show Must Go On".to_string());
}

#[test]
fn test_playlist_mover_cancion() {
    let mut p = Playlist::new("playlist1", vec![
        Cancion::new("Smooth Criminal", "Michael Jackson", Genero::POP),
        Cancion::new("Nothing else Matters", "Metallica", Genero::OTROS),
        Cancion::new("The Show Must Go On", "Queen", Genero::ROCK),
        Cancion::new("Eye of the Tiger", "Survivor", Genero::ROCK)
    ]);
    vec![
        Cancion::new("Smooth Criminal", "Michael Jackson", Genero::POP),
        Cancion::new("Nothing else Matters", "Metallica", Genero::OTROS),
        Cancion::new("The Show Must Go On", "Queen", Genero::ROCK),
        Cancion::new("Eye of the Tiger", "Survivor", Genero::ROCK)
    ];
    assert_eq!(p.canciones.len(), 4);
    assert_eq!(p.canciones[0].titulo, "Smooth Criminal".to_string());
    assert!(p.mover_cancion("Smooth Criminal", 0)); // mover al mismo lugar no cambia nada
    assert_eq!(p.canciones[0].titulo, "Smooth Criminal".to_string());
    assert!(p.mover_cancion("Smooth Criminal", 3)); // nothing, show, eye, smooth
    assert_eq!(p.canciones[0].titulo, "Nothing else Matters".to_string());
    assert_eq!(p.canciones[1].titulo, "The Show Must Go On".to_string());
    assert_eq!(p.canciones[2].titulo, "Eye of the Tiger".to_string());
    assert_eq!(p.canciones[3].titulo, "Smooth Criminal".to_string());
    assert!(p.mover_cancion("Eye of the Tiger", 0)); // eye, nothing, show, smooth
    assert_eq!(p.canciones[0].titulo, "Eye of the Tiger".to_string());
    assert_eq!(p.canciones[2].titulo, "The Show Must Go On".to_string());
    assert!(p.mover_cancion("Smooth Criminal", 1)); // eye, smooth, nothing, show
    assert_eq!(p.canciones[1].titulo, "Smooth Criminal".to_string());
    assert_eq!(p.canciones[3].titulo, "The Show Must Go On".to_string());
    assert!(!p.mover_cancion("Smooth Criminal", 10)); // tratar de mover a posicion invalida
    assert!(!p.mover_cancion("Beat It", 1)); // tratar de mover cancion inexistente
}

#[test]
fn test_playlist_buscar_cancion() {
    let p = Playlist::new("playlist1", vec![
        Cancion::new("Smooth Criminal", "Michael Jackson", Genero::POP),
        Cancion::new("Nothing else Matters", "Metallica", Genero::OTROS),
        Cancion::new("The Show Must Go On", "Queen", Genero::ROCK),
        Cancion::new("Eye of the Tiger", "Survivor", Genero::ROCK)
    ]);
    assert_eq!(p.buscar_cancion("Smooth Criminal"), Some(Cancion::new("Smooth Criminal", "Michael Jackson", Genero::POP)));
    assert_eq!(p.buscar_cancion("Eye of the Tiger"), Some(Cancion::new("Eye of the Tiger", "Survivor", Genero::ROCK)));
    assert_eq!(p.buscar_cancion("nothing else matters"), Some(Cancion::new("Nothing else Matters", "Metallica", Genero::OTROS))); // no importan las mayusculas
    assert_eq!(p.buscar_cancion("Beat It"), None);
    assert_eq!(p.buscar_cancion(""), None);
}

#[test]
fn test_playlist_canciones_genero() {
    let p = Playlist::new("playlist1", vec![
        Cancion::new("Smooth Criminal", "Michael Jackson", Genero::POP),
        Cancion::new("Nothing else Matters", "Metallica", Genero::OTROS),
        Cancion::new("The Show Must Go On", "Queen", Genero::ROCK),
        Cancion::new("Eye of the Tiger", "Survivor", Genero::ROCK)
    ]);
    let rock = p.canciones_de_genero(Genero::ROCK);
    assert_eq!(rock.len(), 2);
    assert!(rock.contains(&Cancion::new("The Show Must Go On", "Queen", Genero::ROCK)));
    assert_eq!(p.canciones_de_genero(Genero::OTROS).len(), 1);
    assert_eq!(p.canciones_de_genero(Genero::JAZZ).len(), 0);
}

#[test]
fn test_playlist_canciones_artista() {
    let p = Playlist::new("playlist1", vec![
        Cancion::new("Master of Puppets", "Metallica", Genero::OTROS),
        Cancion::new("Nothing else Matters", "Metallica", Genero::OTROS),
        Cancion::new("Bohemian Rhapsody", "Queen", Genero::ROCK),
        Cancion::new("Burden", "Opeth", Genero::OTROS)
    ]);
    let metallica = p.canciones_de_artista("Metallica");
    assert_eq!(metallica.len(), 2);
    assert!(metallica.contains(&Cancion::new("Nothing else Matters", "Metallica", Genero::OTROS)));
    assert_eq!(p.canciones_de_artista("opeth").len(), 1);
    assert_eq!(p.canciones_de_artista("MICHAEL JACKSON").len(), 0);
}

#[test]
fn test_playlist_modificar_titulo() {
    let mut p = Playlist::new("mi playlist", vec![]);
    assert_eq!(p.nombre.as_str(), "mi playlist");
    p.modificar_titulo("otra playlist");
    assert_eq!(p.nombre.as_str(), "otra playlist");
}

#[test]
fn test_playlist_borrar_canciones() {
    let mut p = Playlist::new("playlist1", vec![
        Cancion::new("Master of Puppets", "Metallica", Genero::OTROS),
        Cancion::new("Nothing else Matters", "Metallica", Genero::OTROS),
        Cancion::new("Bohemian Rhapsody", "Queen", Genero::ROCK),
        Cancion::new("Burden", "Opeth", Genero::OTROS)
    ]);
    assert_eq!(p.canciones.len(), 4);
    p.borrar_canciones();
    assert_eq!(p.canciones.len(), 0);
}