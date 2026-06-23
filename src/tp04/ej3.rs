#![allow(unused)]
use crate::tp03::ej3::Fecha;
use std::collections::BTreeMap;

impl PartialEq for Fecha {
    fn eq(&self, o: &Fecha) -> bool {
        self.equals(o)
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum TipoSuscripcion {
    Basic, Classic, Super
}

impl TipoSuscripcion {
    const ALL: [Self; 3] = [
        TipoSuscripcion::Basic,
        TipoSuscripcion::Classic,
        TipoSuscripcion::Super
    ];

    pub fn incrementar(tipo: &TipoSuscripcion, counts: &mut [usize; 3]) {
        match tipo {
            TipoSuscripcion::Basic => counts[0] += 1,
            TipoSuscripcion::Classic => counts[1] += 1,
            TipoSuscripcion::Super => counts[2] += 1
        }
    }

    fn get_descuento_por_duracion(&self, duracion_meses: u16) -> f32 {
        if duracion_meses < 3 {
            1.00 // sin descuento para 1 y 2 meses
        } else if duracion_meses < 6 {
            0.95 // 5 % para 3 a 5 meses
        } else if duracion_meses < 12 {
            0.9 // 10% para 6 a 11 meses
        } else {
            0.85 // 15% para >=12 meses
        }
    }

    pub fn get_costo_mensual(&self, duracion_meses: u16) -> f32 {
        let descuento = self.get_descuento_por_duracion(duracion_meses);
        match self {
            TipoSuscripcion::Basic => 4.99*descuento,
            TipoSuscripcion::Classic => 9.99*descuento,
            TipoSuscripcion::Super => 17.99*descuento
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Suscripcion {
    costo_mensual: f32,
    duracion_meses: u16,
    fecha_inicio: Fecha,
    tipo: TipoSuscripcion,
    activa: bool
}

impl Suscripcion {
    pub fn new(tipo: TipoSuscripcion, duracion_meses: u16, fecha_inicio: Fecha) -> Self {
        let costo_mensual = tipo.get_costo_mensual(duracion_meses);
        Suscripcion { costo_mensual, duracion_meses, fecha_inicio, tipo, activa: true }
    }

    fn clone_nuevo_tipo(&mut self, tipo: TipoSuscripcion) -> Self {
        self.activa = false;
        Suscripcion::new(tipo, self.duracion_meses, self.fecha_inicio.clone())
    }

    pub fn upgrade(&mut self) -> Result<Self, &'static str> {
        if self.esta_activa() {
            match self.tipo {
                TipoSuscripcion::Basic => Ok(self.clone_nuevo_tipo(TipoSuscripcion::Classic)),
                TipoSuscripcion::Classic => Ok(self.clone_nuevo_tipo(TipoSuscripcion::Super)),
                TipoSuscripcion::Super => Err("No se puede mejorar una suscripcion Super")
            }
        } else {
            Err("Suscripcion no activa")
        }
        
    }

    pub fn downgrade(&mut self) -> Result<Self, &'static str> {
        if self.esta_activa() {
            match self.tipo {
                TipoSuscripcion::Basic => self.cancelar(),
                TipoSuscripcion::Classic => Ok(self.clone_nuevo_tipo(TipoSuscripcion::Basic)),
                TipoSuscripcion::Super => Ok(self.clone_nuevo_tipo(TipoSuscripcion::Classic))
            }
        } else {
            Err("Suscripcion no activa")
        }
    }

    pub fn cancelar(&mut self) -> Result<Self, &'static str> {
        if self.esta_activa() {
            self.activa = false;
            Ok(self.clone())
        } else {
            Err("Suscripcion ya cancelada")
        }
    }

    pub fn esta_activa(&self) -> bool {
        self.activa
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum TipoMedioPago {
    Efectivo,
    MercadoPago,
    TarjetaCredito,
    Transferencia,
    Cripto
}

impl TipoMedioPago {
    const ALL: [Self; 5] = [
        Self::Efectivo,
        Self::MercadoPago,
        Self::TarjetaCredito,
        Self::Transferencia,
        Self::Cripto,
    ];

    pub fn incrementar(medio: &MedioPago, counts: &mut [usize; 5]) {
        match medio {
            MedioPago::Efectivo => counts[0] += 1,
            MedioPago::MercadoPago { .. } => counts[1] += 1,
            MedioPago::TarjetaCredito { .. } => counts[2] += 1,
            MedioPago::Transferencia { .. } => counts[3] += 1,
            MedioPago::Cripto { .. } => counts[4] += 1,
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum MedioPago {
    Efectivo,
    MercadoPago {
        alias: String
    },
    TarjetaCredito {
        numero_tarjeta: String, // 1234-5678-9123-4567
        cvv: u16
    },
    Transferencia {
        numero_cuenta: u64
    },
    Cripto {
        direccion_wallet: String
    }
}

impl MedioPago {
    pub fn es_tipo(&self, tipo: &TipoMedioPago) -> bool {
        matches!((self, tipo), (MedioPago::Efectivo, TipoMedioPago::Efectivo) |
            (MedioPago::MercadoPago { .. }, TipoMedioPago::MercadoPago) |
            (MedioPago::TarjetaCredito { .. }, TipoMedioPago::TarjetaCredito) |
            (MedioPago::Transferencia { .. }, TipoMedioPago::Transferencia) |
            (MedioPago::Cripto { .. }, TipoMedioPago::Cripto))
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Usuario {
    pub id: u64,
    pub nombre: String,
    medio_pago: MedioPago
}

impl Usuario {
    pub fn new(id: u64, nombre: &str, medio_pago: MedioPago) -> Self {
        Usuario { id, nombre: nombre.to_string(), medio_pago }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Plataforma {
    usuarios: BTreeMap<u64, Usuario>,
    suscripciones: BTreeMap<u64, Vec<Suscripcion>>,
    curr_id: u64
}

impl Plataforma {
    pub fn new() -> Self {
        Plataforma { usuarios: BTreeMap::new(), suscripciones: BTreeMap::new(), curr_id: 0 }
    }

    fn get_next_userid(&mut self) -> u64 {
        let tmp = self.curr_id;
        self.curr_id += 1;
        tmp
    }

    pub fn crear_usuario(&mut self, nombre: &str, suscripcion: Suscripcion, medio_pago: MedioPago) {
        let id = self.get_next_userid();
        self.usuarios.insert(id, Usuario::new(id, nombre, medio_pago));
        self.suscripciones.insert(id, vec![suscripcion]);
    }

    pub fn upgrade_suscripcion(&mut self, id: u64) -> Result<(), &'static str> {
        if let Some(vecsusc) = self.suscripciones.get_mut(&id) {
            let newsusc = vecsusc.last_mut().unwrap().upgrade()?;
            vecsusc.push(newsusc); 
            Ok(())
        } else {
            Err("Usuario no existente")
        }
    }

    pub fn downgrade_suscripcion(&mut self, id: u64) -> Result<(), &'static str> {
        if let Some(vecsusc) = self.suscripciones.get_mut(&id) {
            let newsusc = vecsusc.last_mut().unwrap().downgrade()?;
            if newsusc.esta_activa() { vecsusc.push(newsusc); }
            Ok(())
        } else {
            Err("Usuario no existente")
        }
    }

    pub fn cancelar_suscripcion(&mut self, id: u64) -> Result<(), &'static str> {
        if let Some(vecsusc) = self.suscripciones.get_mut(&id) {
            vecsusc.last_mut().unwrap().cancelar()?;
            Ok(())
        } else {
            Err("Usuario no existente")
        }
    }

    fn tiene_suscripcion_activa(&self, id: u64) -> bool {
        self.suscripciones.get(&id).unwrap().last().unwrap().esta_activa()
    }

    pub fn medio_pago_mas_usado(&self) -> TipoMedioPago {
        let mut counts: [usize; 5] = [0; 5];
        for user in self.usuarios.values() {
            TipoMedioPago::incrementar(&user.medio_pago, &mut counts);
        }
        
        let maxpos = counts.iter().enumerate().max_by_key(|&(_, v)| v).unwrap().0;
        TipoMedioPago::ALL[maxpos]
    }

    pub fn medio_pago_mas_usado_activas(&self) -> TipoMedioPago {
        let mut counts: [usize; 5] = [0; 5];
        for user in self.usuarios.iter()
                .filter_map(|(uid, user)| {
                    if self.tiene_suscripcion_activa(*uid) { Some(user) } else { None }
                }) {
            TipoMedioPago::incrementar(&user.medio_pago, &mut counts);
        }
        
        let maxpos = counts.iter().enumerate().max_by_key(|&(_, v)| v).unwrap().0;
        TipoMedioPago::ALL[maxpos]
    }

    pub fn suscripcion_mas_contratada(&self) -> TipoSuscripcion {
        let mut counts: [usize; 3] = [0; 3];
        for susc in self.suscripciones.values().flatten() {
            TipoSuscripcion::incrementar(&susc.tipo, &mut counts);
        }
        let maxpos = counts.iter().enumerate().max_by_key(|&(_, v)| v).unwrap().0;
        TipoSuscripcion::ALL[maxpos]
    }

    pub fn suscripcion_mas_contratada_activas(&self) -> TipoSuscripcion {
        let mut counts: [usize; 3] = [0; 3];
        for susc in self.suscripciones.values()
                    .filter_map(|vecsusc| {
                        let s = vecsusc.last().unwrap();
                        if s.esta_activa() { Some(s) } else { None }
                    }) {
            TipoSuscripcion::incrementar(&susc.tipo, &mut counts);
        }
        let maxpos = counts.iter().enumerate().max_by_key(|&(_, v)| v).unwrap().0;
        TipoSuscripcion::ALL[maxpos]
    }

    pub fn cant_usuarios(&self) -> usize {
        self.usuarios.len()
    }

    pub fn cant_suscripciones(&self) -> usize {
        self.suscripciones.values().flatten().count()
    }

    pub fn cant_suscripciones_activas(&self) -> usize {
        self.suscripciones.values().filter_map(|vecsusc| {
            let s = vecsusc.last().unwrap();
            if s.esta_activa() { Some(s) } else { None }
        }).count()
    }

    pub fn get_user(&self, id: u64) -> Option<&Usuario> {
        self.usuarios.get(&id)
    }

    pub fn get_user_suscripciones(&self, id: u64) -> Option<&Vec<Suscripcion>> {
        self.suscripciones.get(&id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn crear_plataforma_base() -> Plataforma {
        let mut p = Plataforma::new();
        p.crear_usuario(
            "user1", 
            Suscripcion::new(TipoSuscripcion::Classic, 3, Fecha::new(17, 6, 2026)),
            MedioPago::TarjetaCredito { numero_tarjeta: "4459-2323-0091-8928".to_string(), cvv: 621 }
        );
        p.crear_usuario(
            "pedro56", 
            Suscripcion::new(TipoSuscripcion::Basic, 6, Fecha::new(12, 2, 2026)),
            MedioPago::MercadoPago { alias: "pedro.martinez56.mp".to_string() }
        );
        p.crear_usuario(
            "juan.diaz1", 
            Suscripcion::new(TipoSuscripcion::Super, 12, Fecha::new(3, 8, 2025)),
            MedioPago::Cripto { direccion_wallet: "0x73210aDxnUD9A90JD82".to_string() }
        );
        p
    }

    #[test]
    fn test_crear_usuario() {
        let mut p = crear_plataforma_base();
        assert_eq!(p.cant_usuarios(), 3);
        assert_eq!(p.cant_suscripciones(), 3);

        let u1 = p.get_user(0);
        assert!(u1.is_some_and(|u| u.nombre == "user1" && u.medio_pago.es_tipo(&TipoMedioPago::TarjetaCredito) && !u.medio_pago.es_tipo(&TipoMedioPago::MercadoPago)));

        let u2 = p.get_user(1);
        assert!(u2.is_some_and(|u| u.nombre == "pedro56" && u.medio_pago.es_tipo(&TipoMedioPago::MercadoPago)));

        let u3 = p.get_user(2);
        assert!(u3.is_some_and(|u| u.nombre == "juan.diaz1" && u.medio_pago.es_tipo(&TipoMedioPago::Cripto)));
    
        assert!(p.get_user(3).is_none());
        assert_eq!(p.get_user_suscripciones(0).unwrap().first().unwrap().fecha_inicio, Fecha::new(17, 6, 2026));
        assert!(p.get_user_suscripciones(1).unwrap().first().unwrap().fecha_inicio != Fecha::new(17, 6, 2026));
    }

    #[test]
    fn test_upgrade_suscripcion() {
        let mut p = crear_plataforma_base();

        assert!(p.get_user_suscripciones(0).unwrap().first().is_some_and(|s| s.tipo == TipoSuscripcion::Classic));

        // upgrade Classic a Super
        assert!(p.upgrade_suscripcion(0).is_ok());
        let susc0 = p.get_user_suscripciones(0).unwrap();
        assert!(susc0.first().is_some_and(|s| s.tipo == TipoSuscripcion::Classic && !s.esta_activa()));
        assert!(susc0.get(1).is_some_and(|s| s.tipo == TipoSuscripcion::Super && s.esta_activa()));

        // intentar upgrade de Super falla
        assert!(p.upgrade_suscripcion(0).is_err());

        // se guardan todas las suscripciones, pero no se agregan nuevas activas (1 por usuario)
        assert_eq!(p.cant_suscripciones_activas(), p.cant_usuarios());
        assert_eq!(p.cant_suscripciones(), p.cant_usuarios()+1);

        // upgrade Basic a Classic
        assert!(p.upgrade_suscripcion(1).is_ok());
        let susc1 = p.get_user_suscripciones(1).unwrap();
        assert!(susc1.first().is_some_and(|s| s.tipo == TipoSuscripcion::Basic && !s.esta_activa()));
        assert!(susc1.get(1).is_some_and(|s| s.tipo == TipoSuscripcion::Classic && s.esta_activa()));

        assert_eq!(p.cant_suscripciones_activas(), p.cant_usuarios());
        assert_eq!(p.cant_suscripciones(), p.cant_usuarios()+2);

        // upgrade a suscripcion no activa
        p.cancelar_suscripcion(0);
        assert!(p.upgrade_suscripcion(0).is_err());

        // upgrade a usuario no existente
        assert!(p.upgrade_suscripcion(100).is_err());
    }

    #[test]
    fn test_downgrade_suscripcion() {
        let mut p = crear_plataforma_base();

        // downgrade Classic a Basic
        assert!(p.downgrade_suscripcion(0).is_ok());
        let susc0 = p.get_user_suscripciones(0).unwrap();
        assert!(susc0.first().is_some_and(|s| s.tipo == TipoSuscripcion::Classic && !s.esta_activa()));
        assert!(susc0.get(1).is_some_and(|s| s.tipo == TipoSuscripcion::Basic && s.esta_activa()));

        // downgrade Basic cancela, sin agregar al historial una nueva entrada
        assert!(p.downgrade_suscripcion(0).is_ok());
        let susc0 = p.get_user_suscripciones(0).unwrap();
        assert!(susc0.first().is_some_and(|s| s.tipo == TipoSuscripcion::Classic && !s.esta_activa()));
        assert!(susc0.get(1).is_some_and(|s| s.tipo == TipoSuscripcion::Basic && !s.esta_activa()));
        assert!(susc0.get(2).is_none());

        // intentar downgrade con cancelada falla
        assert!(p.downgrade_suscripcion(0).is_err());

        // hay 3 usuarios, pero 1 cancelo
        // 1 suscripcion activa a lo sumo por usuario, pero se guarda el historial
        assert_eq!(p.cant_suscripciones_activas(), p.cant_usuarios()-1);
        assert_eq!(p.cant_suscripciones(), p.cant_usuarios()+1);

        // downgrade Super a Classic
        assert!(p.downgrade_suscripcion(2).is_ok());
        let susc2 = p.get_user_suscripciones(2).unwrap();
        assert!(susc2.first().is_some_and(|s| s.tipo == TipoSuscripcion::Super && !s.esta_activa()));
        assert!(susc2.get(1).is_some_and(|s| s.tipo == TipoSuscripcion::Classic && s.esta_activa()));

        assert_eq!(p.cant_suscripciones_activas(), p.cant_usuarios()-1);
        assert_eq!(p.cant_suscripciones(), p.cant_usuarios()+2);

        // downgrade a usuario no existente
        assert!(p.downgrade_suscripcion(100).is_err());
    }

    #[test]
    fn test_cancelar_suscripcion() {
        let mut p = crear_plataforma_base();

        assert!(p.get_user_suscripciones(0).unwrap().first().is_some_and(|s| s.tipo == TipoSuscripcion::Classic));

        // cancelar suscripcion de usuario 0 (no agrega al historial)
        assert!(p.cancelar_suscripcion(0).is_ok());
        let susc0 = p.get_user_suscripciones(0).unwrap();
        assert!(susc0.first().is_some_and(|s| s.tipo == TipoSuscripcion::Classic && !s.esta_activa()));
        assert!(susc0.get(1).is_none());

        // intentar cancelar de nuevo falla
        assert!(p.cancelar_suscripcion(0).is_err());

        // hay 3 usuarios, 1 cancelo, no cambia la cantidad de suscripciones totales
        assert_eq!(p.cant_suscripciones_activas(), p.cant_usuarios()-1);
        assert_eq!(p.cant_suscripciones(), p.cant_usuarios());

        // cancelo a usuario no existente
        assert!(p.cancelar_suscripcion(100).is_err());
    }

    #[test]
    fn test_medio_pago_mas_usado_activas() {
        let mut p = crear_plataforma_base();
        p.crear_usuario(
            "maria.a13", 
            Suscripcion::new(TipoSuscripcion::Classic, 7, Fecha::new(25, 4, 2026)),
            MedioPago::Cripto { direccion_wallet: "0x834hDUjd9k29dna7DH9".to_string() }
        );

        // id 0: TarjetaCredito (activo)
        // id 1: MercadoPago (activo)
        // id 2: Cripto (activo)
        // id 3: Cripto (activo)
        // mas usado activo -> Cripto
        let mp = p.medio_pago_mas_usado_activas();
        assert_eq!(mp, TipoMedioPago::Cripto);

        assert!(p.cancelar_suscripcion(2).is_ok());
        assert!(p.cancelar_suscripcion(3).is_ok());

        p.crear_usuario(
            "carlos_sanchez_12", 
            Suscripcion::new(TipoSuscripcion::Basic, 4, Fecha::new(13, 5, 2026)),
            MedioPago::MercadoPago { alias: "carlos.sanchez.mp".to_string() }
        );
        p.crear_usuario(
            "messi.crack10", 
            Suscripcion::new(TipoSuscripcion::Super, 1, Fecha::new(2, 6, 2025)),
            MedioPago::Transferencia { numero_cuenta: 1234567 }
        );
        p.crear_usuario(
            "garcia.perez2", 
            Suscripcion::new(TipoSuscripcion::Classic, 26, Fecha::new(29, 4, 2026)),
            MedioPago::Efectivo
        );

        // id 0: TarjetaCredito (activo)
        // id 1: MercadoPago (activo)
        // id 2: Cripto (inactivo)
        // id 3: Cripto (inactivo)
        // id 4: MercadoPago (activo)
        // id 5: Transferencia (inactivo)
        // id 6: Efectivo (inactivo)
        // mas usado activo -> MercadoPago
        let mp = p.medio_pago_mas_usado_activas();
        assert_eq!(mp, TipoMedioPago::MercadoPago);
    }

    #[test]
    fn test_medio_pago_mas_usado() {
        let mut p = crear_plataforma_base();
        p.crear_usuario(
            "maria.a13", 
            Suscripcion::new(TipoSuscripcion::Classic, 7, Fecha::new(25, 4, 2026)),
            MedioPago::Cripto { direccion_wallet: "0x834hDUjd9k29dna7DH9".to_string() }
        );

        // id 0: TarjetaCredito (activo)
        // id 1: MercadoPago (activo)
        // id 2: Cripto (activo)
        // id 3: Cripto (activo)
        // mas usado general -> Cripto
        let mp = p.medio_pago_mas_usado();
        assert_eq!(mp, TipoMedioPago::Cripto);

        assert!(p.cancelar_suscripcion(1).is_ok());
        assert!(p.cancelar_suscripcion(3).is_ok());

        p.crear_usuario(
            "carlos_sanchez_12", 
            Suscripcion::new(TipoSuscripcion::Basic, 4, Fecha::new(13, 5, 2026)),
            MedioPago::MercadoPago { alias: "carlos.sanchez.mp".to_string() }
        );
        p.crear_usuario(
            "leonardo.da.vinci_9", 
            Suscripcion::new(TipoSuscripcion::Super, 24, Fecha::new(1, 2, 2025)),
            MedioPago::MercadoPago { alias: "leonardito.mp".to_string() }
        );
        assert!(p.cancelar_suscripcion(4).is_ok());
        assert!(p.cancelar_suscripcion(5).is_ok());

        // id 0: TarjetaCredito (activo)
        // id 1: MercadoPago (inactivo)
        // id 2: Cripto (activo)
        // id 3: Cripto (inactivo)
        // id 4: MercadoPago (inactivo)
        // id 5: MercadoPago (inactivo)
        // mas usado general -> MercadoPago
        let mp = p.medio_pago_mas_usado();
        assert_eq!(mp, TipoMedioPago::MercadoPago);
    }

    #[test]
    fn test_suscripcion_mas_contratada_activas() {
        let mut p = crear_plataforma_base();
        p.crear_usuario(
            "maria.a13", 
            Suscripcion::new(TipoSuscripcion::Classic, 7, Fecha::new(25, 4, 2026)),
            MedioPago::Cripto { direccion_wallet: "0x834hDUjd9k29dna7DH9".to_string() }
        );

        // id 0: Classic (activo)
        // id 1: Basic (activo)
        // id 2: Super (activo)
        // id 3: Classic (activo)
        // mas usado activo -> Classic
        let sus = p.suscripcion_mas_contratada_activas();
        assert_eq!(sus, TipoSuscripcion::Classic);

        assert!(p.cancelar_suscripcion(2).is_ok());
        assert!(p.cancelar_suscripcion(3).is_ok());

        p.crear_usuario(
            "carlos_sanchez_12", 
            Suscripcion::new(TipoSuscripcion::Basic, 4, Fecha::new(13, 5, 2026)),
            MedioPago::MercadoPago { alias: "carlos.sanchez.mp".to_string() }
        );

        // id 0: Classic (activo)
        // id 1: Basic (activo)
        // id 2: Super (inactivo)
        // id 3: Classic (inactivo)
        // id 4: Basic (activo)
        // mas usado activo -> Classic
        let sus = p.suscripcion_mas_contratada_activas();
        assert_eq!(sus, TipoSuscripcion::Basic);
    }

    #[test]
    fn test_suscripcion_mas_contratada() {
        let mut p = crear_plataforma_base();
        p.crear_usuario(
            "maria.a13", 
            Suscripcion::new(TipoSuscripcion::Classic, 7, Fecha::new(25, 4, 2026)),
            MedioPago::Cripto { direccion_wallet: "0x834hDUjd9k29dna7DH9".to_string() }
        );

        // id 0: Classic (activo)
        // id 1: Basic (activo)
        // id 2: Super (activo)
        // id 3: Classic (activo)
        // mas usado general -> Classic
        let mp = p.suscripcion_mas_contratada();
        assert_eq!(mp, TipoSuscripcion::Classic);

        assert!(p.cancelar_suscripcion(1).is_ok());

        p.crear_usuario(
            "carlos_sanchez_12", 
            Suscripcion::new(TipoSuscripcion::Basic, 4, Fecha::new(13, 5, 2026)),
            MedioPago::MercadoPago { alias: "carlos.sanchez.mp".to_string() }
        );
        p.crear_usuario(
            "leonardo.da.vinci_9", 
            Suscripcion::new(TipoSuscripcion::Basic, 2, Fecha::new(1, 2, 2025)),
            MedioPago::MercadoPago { alias: "leonardito.mp".to_string() }
        );
        assert!(p.cancelar_suscripcion(4).is_ok());
        assert!(p.cancelar_suscripcion(5).is_ok());

        // id 0: Classic (activo)
        // id 1: Basic (inactivo)
        // id 2: Super (activo)
        // id 3: Classic (activo)
        // id 4: Basic (inactivo)
        // id 5: Basic (inactivo)
        // mas usado general -> Basic
        let mp = p.suscripcion_mas_contratada();
        assert_eq!(mp, TipoSuscripcion::Basic);
    }
}






