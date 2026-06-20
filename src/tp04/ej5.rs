#![allow(unused)]
use std::collections::{BTreeMap, HashMap};
use crate::tp03::ej3::Fecha;

#[derive(PartialEq, Debug, Clone)]
pub struct Usuario {
    pub nombre: String,
    pub apellido: String,
    pub email: String,
    dni: u32,
    id_valido: bool
}

impl Usuario {
    pub fn new(nombre: &str, apellido: &str, email: &str, dni: u32) -> Self {
        Usuario { nombre: nombre.to_string(), apellido: apellido.to_string(), email: email.to_string(), dni, id_valido: false }
    }

    pub fn validar_identidad(&mut self) {
        self.id_valido = true;
    }

    pub fn identidad_validada(&self) -> bool {
        self.id_valido
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Blockchain {
    pub nombre: String,
    pub prefijo: String
}

impl Blockchain {
    pub fn get_hash(&self) -> String {
        self.nombre.clone() + &0x1234_5678_9ABC_DEF0_u64.to_string()
    }

    pub fn bitcoin() -> Self {
        Blockchain { nombre: "Bitcoin".to_string(), prefijo: "BTC".to_string() }
    }

    pub fn ethereum() -> Self {
        Blockchain { nombre: "Ethereum".to_string(), prefijo: "ETH".to_string() }
    }

    pub fn tron() -> Self {
        Blockchain { nombre: "Tron".to_string(), prefijo: "TRX".to_string() }
    }

    pub fn solana() -> Self {
        Blockchain { nombre: "Solana".to_string(), prefijo: "SOL".to_string() }
    }

    pub fn polygon() -> Self {
        Blockchain { nombre: "Polygon".to_string(), prefijo: "POL".to_string() }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Criptomoneda {
    pub nombre: String,
    pub prefijo: String,
    blockchains: Vec<Blockchain>
}

impl Criptomoneda {
    const DINERO_FIAT: &str = "DINERO_FIAT";
    const BITCOIN: &str = "BTC";
    const TETHER: &str = "USDT";
    const ETHEREUM: &str = "ETH";
    const USDCOIN: &str = "USDC";
    const SHIBAINU: &str = "SHIB";
    const PREFIJOS: [&str; 6] = [
        Criptomoneda::DINERO_FIAT,
        Criptomoneda::BITCOIN,
        Criptomoneda::TETHER,
        Criptomoneda::ETHEREUM,
        Criptomoneda::USDCOIN,
        Criptomoneda::SHIBAINU
    ];

    pub fn lista_criptos() -> Vec<Criptomoneda> {
        vec![Criptomoneda::bitcoin(), Criptomoneda::tether(), Criptomoneda::ethereum(),
             Criptomoneda::usdcoin(), Criptomoneda::shibainu()]
    }

    pub fn bitcoin() -> Self {
        Criptomoneda { nombre: "Bitcoin".to_string(), prefijo: Criptomoneda::BITCOIN.to_string(), blockchains: vec![Blockchain::bitcoin()]}
    }

    pub fn tether() -> Self {
        Criptomoneda { nombre: "Tether".to_string(), prefijo: Criptomoneda::TETHER.to_string(), blockchains: vec![Blockchain::ethereum(), Blockchain::tron(), Blockchain::solana(), Blockchain::polygon()]}
    }

    pub fn ethereum() -> Self {
        Criptomoneda { nombre: "Ethereum".to_string(), prefijo: Criptomoneda::ETHEREUM.to_string(), blockchains: vec![Blockchain::ethereum()]}
    }

    pub fn usdcoin() -> Self {
        Criptomoneda { nombre: "USD Coin".to_string(), prefijo: Criptomoneda::USDCOIN.to_string(), blockchains: vec![Blockchain::ethereum(), Blockchain::solana(), Blockchain::polygon()]}
    }

    pub fn shibainu() -> Self {
        Criptomoneda { nombre: "Shiba Inu".to_string(), prefijo: Criptomoneda::SHIBAINU.to_string(), blockchains: vec![Blockchain::ethereum(), Blockchain::polygon()]}
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Balance {
    map: HashMap<String, f64> // prefijo cripto/fiat -- balance
}

impl Balance {
    pub fn default() -> Self {
        Balance { map: HashMap::from(Criptomoneda::PREFIJOS.map(|p| (p.to_string(), 0.0))) } 
    }

    pub fn tiene_suficiente(&mut self, prefijo: &str, monto: f64) -> bool {
        *self.map.entry(prefijo.to_string()).or_insert(0.0) >= monto
    }

    pub fn get(&self, prefijo: &str) -> Option<f64> {
        self.map.get(&prefijo.to_string()).copied()
    }

    pub fn agregar(&mut self, prefijo: &str, monto: f64) {
        *self.map.entry(prefijo.to_string()).or_insert(0.0) += monto;
    }

    pub fn quitar(&mut self, prefijo: &str, monto: f64) {
        self.agregar(prefijo, -monto);
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum MedioPago {
    MercadoPago, Transferencia
}

#[derive(PartialEq, Debug, Clone)]
pub enum Transaccion {
    IngresoDinero {
        fecha: Fecha,
        monto: f64,
        usuario: Usuario,
    },
    RetiroDinero {
        fecha: Fecha,
        monto: f64,
        usuario: Usuario,
        medio: MedioPago
    }, 
    CompraCripto {
        fecha: Fecha,
        criptomoneda: String,
        monto: f64,
        usuario: Usuario,
        cotizacion: f64
    }, 
    VentaCripto {
        fecha: Fecha,
        criptomoneda: String,
        monto: f64,
        usuario: Usuario,
        cotizacion: f64
    }, 
    RetiroCripto {
        fecha: Fecha,
        criptomoneda: String,
        monto: f64,
        usuario: Usuario,
        cotizacion: f64,
        blockchain: Blockchain,
        hash: String
    },
    RecepcionCripto {
        fecha: Fecha,
        criptomoneda: String,
        monto: f64,
        usuario: Usuario,
        cotizacion: f64,
        blockchain: Blockchain,
    }
}

impl Transaccion {
    pub fn validar_ingreso_dinero(&self, dni_validar: u32, monto_validar: f64) -> bool {
        match self {
            Transaccion::IngresoDinero { monto, usuario, .. } => 
                *monto == monto_validar && usuario.dni == dni_validar,
            _ => false
        }
    }

    pub fn validar_retiro_dinero(&self, dni_validar: u32, monto_validar: f64, medio_validar: MedioPago) -> bool {
        match self {
            Transaccion::RetiroDinero { monto, usuario, medio, .. } => 
                *monto == monto_validar && usuario.dni == dni_validar && *medio == medio_validar,
            _ => false
        }
    }

    pub fn validar_compra_cripto(&self, dni_validar: u32, monto_validar: f64, cripto_validar: &str, cotizacion_validar: f64) -> bool {
        match self {
            Transaccion::CompraCripto { monto, usuario, criptomoneda, cotizacion, .. } => 
                *monto == monto_validar && usuario.dni == dni_validar && *criptomoneda == cripto_validar && *cotizacion == cotizacion_validar,
            _ => false
        }
    }

    pub fn validar_venta_cripto(&self, dni_validar: u32, monto_validar: f64, cripto_validar: &str, cotizacion_validar: f64) -> bool {
        match self {
            Transaccion::VentaCripto { monto, usuario, criptomoneda, cotizacion, .. } => 
                *monto == monto_validar && usuario.dni == dni_validar && *criptomoneda == cripto_validar && *cotizacion == cotizacion_validar,
            _ => false
        }
    }

    pub fn validar_retiro_cripto(&self, dni_validar: u32, monto_validar: f64, cripto_validar: &str, cotizacion_validar: f64, blockchain_validar: &str, hash_validar: String) -> bool {
        match self {
            Transaccion::RetiroCripto { monto, usuario, criptomoneda, cotizacion, hash, blockchain,  .. } => 
                *monto == monto_validar && usuario.dni == dni_validar && *criptomoneda == cripto_validar && *cotizacion == cotizacion_validar && *hash == hash_validar && blockchain.prefijo == blockchain_validar,
            _ => false
        }
    }

    pub fn validar_recepcion_cripto(&self, dni_validar: u32, monto_validar: f64, cripto_validar: &str, cotizacion_validar: f64, blockchain_validar: &str) -> bool {
        match self {
            Transaccion::RecepcionCripto { monto, usuario, criptomoneda, cotizacion, blockchain, .. } => 
                *monto == monto_validar && usuario.dni == dni_validar && *criptomoneda == cripto_validar && *cotizacion == cotizacion_validar && blockchain.prefijo == blockchain_validar,
            _ => false
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum ErrorTransaccion {
    UsuarioInexistente, MontoNegativo, BalanceInsuficiente, CriptoInexistente, UsuarioNoValidado, BlockchainInvalida
}

#[derive(PartialEq, Debug, Clone)]
pub enum ErrorUsuario {
    UsuarioYaExiste, UsuarioInexistente
}

pub struct PlataformaXYZ {
    usuarios: BTreeMap<u32, Usuario>, // dni -- usuario
    transacciones: BTreeMap<u32, Vec<Transaccion>>, // dni -- historial de transacciones del usuario
    balances: BTreeMap<u32, Balance>, // dni -- balance de las distintas criptos del usuario
    criptomonedas: Vec<Criptomoneda>
}

impl PlataformaXYZ {
    pub fn new() -> Self {
        PlataformaXYZ { usuarios: BTreeMap::new(), balances: BTreeMap::new(), transacciones: BTreeMap::new(), criptomonedas: Criptomoneda::lista_criptos() }
    }

    pub fn get_cotizacion(prefijo: &str) -> f64 {
        // devuelve el valor (en dolares) de 1 unidad de la criptomoneda
        match prefijo {
            Criptomoneda::BITCOIN => 63030.62,
            Criptomoneda::TETHER => 1.0,
            Criptomoneda::ETHEREUM => 1700.99,
            Criptomoneda::USDCOIN => 1.0,
            Criptomoneda::SHIBAINU => 0.000004735,
            _ => 1.0
        }
    }

    pub fn registrar_usuario(&mut self, usuario: Usuario) -> Result<(), ErrorUsuario> {
        let dni = usuario.dni;
        if self.usuarios.contains_key(&dni) {
            Err(ErrorUsuario::UsuarioYaExiste)
        } else {
            self.usuarios.insert(dni, usuario);
            self.transacciones.insert(dni, vec![]);
            self.balances.insert(dni, Balance::default());
            Ok(())
        }
    }

    pub fn validar_usuario(&mut self, dni: u32) -> Result<(), ErrorUsuario> {
        if let Some(user) = self.usuarios.get_mut(&dni) {
            user.validar_identidad();
            Ok(())
        } else {
            Err(ErrorUsuario::UsuarioInexistente)
        }
    }

    pub fn get_usuario(&self, dni: u32) -> Result<&Usuario, ErrorUsuario> {
        if let Some(user) = self.usuarios.get(&dni) {
            Ok(&user)
        } else {
            Err(ErrorUsuario::UsuarioInexistente)
        }
    }

    pub fn get_balance_usuario(&self, dni: u32, prefijo: &str) -> Result<Option<f64>, ErrorUsuario> {
        if let Some(balance) = self.balances.get(&dni) {
            Ok(balance.get(prefijo))
        } else {
            Err(ErrorUsuario::UsuarioInexistente)
        }
    }

    pub fn ingresar_dinero(&mut self, dni: u32, monto: f64) -> Result<Transaccion, ErrorTransaccion> {
        if monto < 0.0 {
            Err(ErrorTransaccion::MontoNegativo)
        } else if let Some(user) = self.usuarios.get(&dni) {
            if !user.identidad_validada() {
                return Err(ErrorTransaccion::UsuarioNoValidado);
            }

            let balance = self.balances.get_mut(&dni).unwrap();
            balance.agregar(Criptomoneda::DINERO_FIAT, monto);

            let t = Transaccion::IngresoDinero {
                fecha: Fecha::fecha_actual(),
                monto: monto,
                usuario: user.clone()
            };
            // si existe el usuario en self.usuarios, tambien debe existir en self.transacciones
            // ya que se crean al mismo tiempo en registrar_usuario
            self.transacciones.get_mut(&dni).unwrap().push(t.clone());
            Ok(t)
        } else {
            Err(ErrorTransaccion::UsuarioInexistente)
        }
    }

    pub fn retirar_dinero(&mut self, dni: u32, monto: f64, medio: MedioPago) -> Result<Transaccion, ErrorTransaccion> {
        if monto < 0.0 {
            Err(ErrorTransaccion::MontoNegativo)
        } else if let Some(user) = self.usuarios.get(&dni) {
            if !user.identidad_validada() {
                return Err(ErrorTransaccion::UsuarioNoValidado);
            }
            
            let balance = self.balances.get_mut(&dni).unwrap();
            if !balance.tiene_suficiente(Criptomoneda::DINERO_FIAT, monto) {
                return Err(ErrorTransaccion::BalanceInsuficiente);
            }
            balance.quitar(Criptomoneda::DINERO_FIAT, monto);

            let t = Transaccion::RetiroDinero {
                fecha: Fecha::fecha_actual(),
                monto: monto,
                usuario: user.clone(),
                medio: medio
            };
            self.transacciones.get_mut(&dni).unwrap().push(t.clone());
            Ok(t)
        } else {
            Err(ErrorTransaccion::UsuarioInexistente)
        }
    }

    pub fn comprar_cripto(&mut self, dni: u32, cantidad: f64, prefijo: &str) -> Result<Transaccion, ErrorTransaccion> {
        if cantidad < 0.0 {
            Err(ErrorTransaccion::MontoNegativo)
        } else if !self.criptomonedas.iter().any(|c| c.prefijo == prefijo) {
            Err(ErrorTransaccion::CriptoInexistente)
        } else if let Some(user) = self.usuarios.get(&dni) {
            if !user.identidad_validada() {
                return Err(ErrorTransaccion::UsuarioNoValidado);
            }
            
            let cotizacion = PlataformaXYZ::get_cotizacion(prefijo);
            let balance = self.balances.get_mut(&dni).unwrap();
            let monto_a_pagar = cantidad*cotizacion;
            if !balance.tiene_suficiente(Criptomoneda::DINERO_FIAT, monto_a_pagar) {
                return Err(ErrorTransaccion::BalanceInsuficiente);
            }
            balance.quitar(Criptomoneda::DINERO_FIAT, monto_a_pagar);
            balance.agregar(prefijo, cantidad);
            
            let t = Transaccion::CompraCripto { 
                fecha: Fecha::fecha_actual(),
                criptomoneda: prefijo.to_string(), 
                monto: cantidad, 
                usuario: user.clone(), 
                cotizacion: cotizacion 
            };
            self.transacciones.get_mut(&dni).unwrap().push(t.clone());
            Ok(t)
        } else {
            Err(ErrorTransaccion::UsuarioInexistente)
        }
    }

    pub fn vender_cripto(&mut self, dni: u32, cantidad: f64, prefijo: &str) -> Result<Transaccion, ErrorTransaccion> {
        if cantidad < 0.0 {
            Err(ErrorTransaccion::MontoNegativo)
        } else if !self.criptomonedas.iter().any(|c| c.prefijo == prefijo) {
            Err(ErrorTransaccion::CriptoInexistente)
        } else if let Some(user) = self.usuarios.get(&dni) {
            if !user.identidad_validada() {
                return Err(ErrorTransaccion::UsuarioNoValidado);
            }
            
            let cotizacion = PlataformaXYZ::get_cotizacion(prefijo);
            let balance = self.balances.get_mut(&dni).unwrap();
            if !balance.tiene_suficiente(prefijo, cantidad) {
                return Err(ErrorTransaccion::BalanceInsuficiente);
            }
            balance.quitar(prefijo, cantidad);
            balance.agregar(Criptomoneda::DINERO_FIAT, cantidad*cotizacion);
            
            let t = Transaccion::VentaCripto { 
                fecha: Fecha::fecha_actual(),
                criptomoneda: prefijo.to_string(), 
                monto: cantidad, 
                usuario: user.clone(), 
                cotizacion: cotizacion 
            };
            self.transacciones.get_mut(&dni).unwrap().push(t.clone());
            Ok(t)
        } else {
            Err(ErrorTransaccion::UsuarioInexistente)
        }
    }

    pub fn retirar_cripto(&mut self, dni: u32, cantidad: f64, prefijo: &str, blockchain: &Blockchain) -> Result<Transaccion, ErrorTransaccion> {
        if cantidad < 0.0 {
            Err(ErrorTransaccion::MontoNegativo)
        } else if !self.criptomonedas.iter().any(|c| c.prefijo == prefijo) {
            Err(ErrorTransaccion::CriptoInexistente)
        } else if let Some(user) = self.usuarios.get(&dni) {
            if !user.identidad_validada() {
                return Err(ErrorTransaccion::UsuarioNoValidado);
            }

            let cm = self.criptomonedas.iter().find(|c| c.prefijo == prefijo).unwrap();
            if !cm.blockchains.iter().any(|b| *b == *blockchain) {
                return Err(ErrorTransaccion::BlockchainInvalida);
            }
            
            let cotizacion = PlataformaXYZ::get_cotizacion(prefijo);
            let balance = self.balances.get_mut(&dni).unwrap();
            if !balance.tiene_suficiente(prefijo, cantidad) {
                return Err(ErrorTransaccion::BalanceInsuficiente);
            }
            balance.quitar(prefijo, cantidad);
            
            let t = Transaccion::RetiroCripto { 
                fecha: Fecha::fecha_actual(),
                criptomoneda: prefijo.to_string(), 
                monto: cantidad, 
                usuario: user.clone(), 
                cotizacion: cotizacion,
                blockchain: blockchain.clone(),
                hash: blockchain.get_hash()
            };
            self.transacciones.get_mut(&dni).unwrap().push(t.clone());
            Ok(t)
        } else {
            Err(ErrorTransaccion::UsuarioInexistente)
        }
    }

    pub fn recibir_cripto(&mut self, dni: u32, cantidad: f64, prefijo: &str, blockchain: &Blockchain) -> Result<Transaccion, ErrorTransaccion> {
        if cantidad < 0.0 {
            Err(ErrorTransaccion::MontoNegativo)
        } else if !self.criptomonedas.iter().any(|c| c.prefijo == prefijo) {
            Err(ErrorTransaccion::CriptoInexistente)
        } else if let Some(user) = self.usuarios.get(&dni) {
            if !user.identidad_validada() {
                return Err(ErrorTransaccion::UsuarioNoValidado);
            }

            let cm = self.criptomonedas.iter().find(|c| c.prefijo == prefijo).unwrap();
            if !cm.blockchains.iter().any(|b| *b == *blockchain) {
                return Err(ErrorTransaccion::BlockchainInvalida);
            }
            
            let cotizacion = PlataformaXYZ::get_cotizacion(prefijo);
            let balance = self.balances.get_mut(&dni).unwrap();
            balance.agregar(prefijo, cantidad);
            
            let t = Transaccion::RecepcionCripto { 
                fecha: Fecha::fecha_actual(),
                criptomoneda: prefijo.to_string(), 
                monto: cantidad, 
                usuario: user.clone(), 
                cotizacion: cotizacion,
                blockchain: blockchain.clone()
            };
            self.transacciones.get_mut(&dni).unwrap().push(t.clone());
            Ok(t)
        } else {
            Err(ErrorTransaccion::UsuarioInexistente)
        }
    }

    fn incrementar_cripto_arr<T: std::ops::AddAssign>(cripto: &str, arr: &mut [T; 5], val: T) {
        match cripto {
            Criptomoneda::BITCOIN => arr[0] += val,
            Criptomoneda::TETHER => arr[1] += val,
            Criptomoneda::ETHEREUM => arr[2] += val,
            Criptomoneda::USDCOIN => arr[3] += val,
            Criptomoneda::SHIBAINU => arr[4] += val,
            v => unreachable!("Valor inesperado: {v}")
        };
    }

    fn get_max_cripto_from_arr<T: PartialOrd>(&self, arr: &[T; 5]) -> Criptomoneda {
        let maxpos = arr.iter().enumerate().max_by(|(i1, v1), (i2, v2)| v1.partial_cmp(v2).unwrap()).unwrap().0;
        let cmprefix = match maxpos {
            0 => Criptomoneda::BITCOIN,
            1 => Criptomoneda::TETHER,
            2 => Criptomoneda::ETHEREUM,
            3 => Criptomoneda::USDCOIN,
            4 => Criptomoneda::SHIBAINU,
            v => unreachable!("Valor inesperado: {v}")
        };
        self.criptomonedas.iter().find(|c| c.prefijo == cmprefix).unwrap().clone()
    }

    pub fn cripto_mas_vendida(&self) -> Criptomoneda {
        let mut cant_ventas_cripto = [0_usize; 5];
        self.transacciones.values().flatten().for_each(|t| 
            if let Transaccion::VentaCripto { criptomoneda, .. } = t {
                PlataformaXYZ::incrementar_cripto_arr(criptomoneda, &mut cant_ventas_cripto, 1);
            }
        );
        self.get_max_cripto_from_arr(&cant_ventas_cripto)
    }

    pub fn cripto_mas_comprada(&self) -> Criptomoneda {
        let mut cant_compras_cripto = [0_usize; 5];
        self.transacciones.values().flatten().for_each(|t| 
            if let Transaccion::CompraCripto { criptomoneda, .. } = t {
                PlataformaXYZ::incrementar_cripto_arr(criptomoneda, &mut cant_compras_cripto, 1);
            }
        );
        self.get_max_cripto_from_arr(&cant_compras_cripto)
    }

    pub fn cripto_mas_volumen_ventas(&self) -> Criptomoneda {
        let mut volumen_ventas_cripto = [0.0; 5];
        self.transacciones.values().flatten().for_each(|t| 
            if let Transaccion::VentaCripto { criptomoneda, monto, cotizacion, .. } = t {
                PlataformaXYZ::incrementar_cripto_arr(criptomoneda, &mut volumen_ventas_cripto, monto*cotizacion);
            }
        );
        self.get_max_cripto_from_arr(&volumen_ventas_cripto)
    }

    pub fn cripto_mas_volumen_compras(&self) -> Criptomoneda {
        let mut volumen_compras_cripto = [0.0; 5];
        self.transacciones.values().flatten().for_each(|t| 
            if let Transaccion::CompraCripto { criptomoneda, monto, cotizacion, .. } = t {
                PlataformaXYZ::incrementar_cripto_arr(criptomoneda, &mut volumen_compras_cripto, monto*cotizacion);
            }
        );
        self.get_max_cripto_from_arr(&volumen_compras_cripto)
    }
}

#[cfg(test)]
mod tests {
    use crate::tp04::ej3::TipoMedioPago::Cripto;

use super::*;

    #[test]
    fn test_registrar_usuario() {
        let mut p = PlataformaXYZ::new();
        assert!(p.registrar_usuario(
            Usuario::new("Pedro", "Perez", "pedro.perez@gmail.com", 41_192_387)
        ).is_ok());
        assert!(p.registrar_usuario(
            Usuario::new("Maria", "Sanchez", "maria.sanchez@hotmail.com", 38_998_761)
        ).is_ok());
        assert!(p.registrar_usuario( // exactamente mismo usuario que el primero
            Usuario::new("Pedro", "Perez", "pedro.perez@gmail.com", 41_192_387)
        ).is_err_and(|e| e == ErrorUsuario::UsuarioYaExiste));
        assert!(p.registrar_usuario( // solo dni igual
            Usuario::new("Juan", "Paredes", "juan.paredes@gmail.com", 41_192_387)
        ).is_err_and(|e| e == ErrorUsuario::UsuarioYaExiste));

        assert!(p.get_usuario(41_192_387).is_ok_and(|u| u.dni == 41_192_387 && u.nombre == "Pedro"));
        assert!(p.get_usuario(38_998_761).is_ok_and(|u| u.dni == 38_998_761 && u.nombre == "Maria"));
        assert!(p.get_usuario(12_345_678).is_err_and(|e| e == ErrorUsuario::UsuarioInexistente));

        assert!(p.get_balance_usuario(41_192_387, Criptomoneda::DINERO_FIAT).is_ok_and(|b| b == Some(0.0)));
        assert!(p.get_balance_usuario(41_192_387, "1234").is_ok_and(|b| b.is_none()));
        assert!(p.get_balance_usuario(12_345_678, Criptomoneda::DINERO_FIAT).is_err_and(|e| e == ErrorUsuario::UsuarioInexistente));
    }

    fn crear_plataforma_base() -> PlataformaXYZ {
        let mut p = PlataformaXYZ::new();
        assert!(p.registrar_usuario(
            Usuario::new("Pedro", "Perez", "pedro.perez@gmail.com", 41_192_387)
        ).is_ok());

        assert!(p.registrar_usuario(
            Usuario::new("Maria", "Sanchez", "maria.sanchez@hotmail.com", 38_998_761)
        ).is_ok());

        assert!(p.registrar_usuario(
            Usuario::new("Tim", "Payne", "tim.payne@gmail.com", 42_298_985)
        ).is_ok());

        assert!(p.registrar_usuario(
            Usuario::new("Juan", "Paredes", "juan.paredes@gmail.com", 44_144_414)
        ).is_ok());
        p
    }

    fn crear_plataforma_base_compras_ventas() -> PlataformaXYZ {
        let mut p = crear_plataforma_base();
        assert!(p.validar_usuario(41_192_387).is_ok());

        assert!(p.ingresar_dinero(41_192_387, 100000000.0).is_ok());
        assert!(p.comprar_cripto(41_192_387, 4.0, Criptomoneda::SHIBAINU).is_ok());
        assert!(p.comprar_cripto(41_192_387, 10.0, Criptomoneda::ETHEREUM).is_ok());
        assert!(p.comprar_cripto(41_192_387, 25.0, Criptomoneda::ETHEREUM).is_ok());
        assert!(p.comprar_cripto(41_192_387, 1.0, Criptomoneda::BITCOIN).is_ok());

        assert!(p.vender_cripto(41_192_387, 1.0, Criptomoneda::SHIBAINU).is_ok());
        assert!(p.vender_cripto(41_192_387, 1.0, Criptomoneda::SHIBAINU).is_ok());
        assert!(p.vender_cripto(41_192_387, 1.0, Criptomoneda::SHIBAINU).is_ok());
        assert!(p.vender_cripto(41_192_387, 35.0, Criptomoneda::ETHEREUM).is_ok());
        assert!(p.vender_cripto(41_192_387, 0.2, Criptomoneda::BITCOIN).is_ok());
        assert!(p.vender_cripto(41_192_387, 0.6, Criptomoneda::BITCOIN).is_ok());

        /*
        cotizaciones:
            Criptomoneda::BITCOIN  => 63030.62
            Criptomoneda::TETHER   => 1.0
            Criptomoneda::ETHEREUM => 1700.99
            Criptomoneda::USDCOIN  => 1.0
            Criptomoneda::SHIBAINU => 0.000004735

        compras:
        - SHIB = 1
        - ETH = 2 (mayor cant compras)
        - BTC = 1

        volumen compras:
        - SHIB = 4*cotizacion_shib ~ $0.00001894
        - ETH = (10+25)*cotizacion_eth ~ $59534.65
        - BTC = 1*cotizacion_btc ~ $63030.62 (mayor volumen compras)

        ventas:
        - SHIB = 3 (mayor cant ventas)
        - ETH = 1
        - BTC = 2

        volumen ventas:
        - SHIB = (1+1+1)*cotizacion_shib ~ $0.000014205
        - ETH = 35*cotizacion_eth ~ $59534.65 (mayor volumen ventas)
        - BTC = (0.2+0.6)*cotizacion_btc ~ $50424.496
        */

        p
    }

    #[test]
    fn test_validar_usuario() {
        let mut p = crear_plataforma_base();
        assert!(p.get_usuario(41_192_387).is_ok_and(|u| !u.identidad_validada()));
        assert!(p.validar_usuario(41_192_387).is_ok() && p.get_usuario(41_192_387).is_ok_and(|u| u.identidad_validada()));
    
        assert!(p.get_usuario(44_144_414).is_ok_and(|u| !u.identidad_validada()));
        assert!(p.validar_usuario(44_144_414).is_ok() && p.get_usuario(44_144_414).is_ok_and(|u| u.identidad_validada()));
    
        assert!(p.validar_usuario(12_345_678).is_err_and(|e| e == ErrorUsuario::UsuarioInexistente));
    }

    #[test]
    fn test_get_cotizacion() {
        assert_eq!(PlataformaXYZ::get_cotizacion(Criptomoneda::BITCOIN), 63030.62);
        assert_eq!(PlataformaXYZ::get_cotizacion(Criptomoneda::TETHER), 1.0);
        assert_eq!(PlataformaXYZ::get_cotizacion(Criptomoneda::ETHEREUM), 1700.99);
        assert_eq!(PlataformaXYZ::get_cotizacion(Criptomoneda::USDCOIN), 1.0);
        assert_eq!(PlataformaXYZ::get_cotizacion(Criptomoneda::SHIBAINU), 0.000004735);
        assert_eq!(PlataformaXYZ::get_cotizacion(Criptomoneda::DINERO_FIAT), 1.0);
    }

    #[test]
    fn test_ingresar_dinero() {
        let mut p = crear_plataforma_base();
        assert!(p.validar_usuario(41_192_387).is_ok());
        assert!(p.validar_usuario(42_298_985).is_ok());
        assert!(p.validar_usuario(44_144_414).is_ok());

        assert!(p.ingresar_dinero(41_192_387, 999.0).is_ok_and(
            |t| t.validar_ingreso_dinero(41_192_387, 999.0) && 
                !t.validar_compra_cripto(41_192_387, 999.0, "", 0.0)
        ));
        assert!(p.get_balance_usuario(41_192_387, Criptomoneda::DINERO_FIAT).is_ok_and(|b| b == Some(999.0)));
        assert!(p.get_balance_usuario(41_192_387, Criptomoneda::BITCOIN).is_ok_and(|b| b == Some(0.0))); // comprobar que no haya afectado a otros balances

        assert!(p.ingresar_dinero(41_192_387, 150.0).is_ok_and(
            |t| t.validar_ingreso_dinero(41_192_387, 150.0)
        ));
        assert!(p.get_balance_usuario(41_192_387, Criptomoneda::DINERO_FIAT).is_ok_and(|b| b == Some(999.0 + 150.0)));

        assert!(p.ingresar_dinero(44_144_414, 58235.0).is_ok_and(
            |t| t.validar_ingreso_dinero(44_144_414, 58235.0)
        ));
        assert!(p.get_balance_usuario(44_144_414, Criptomoneda::DINERO_FIAT).is_ok_and(|b| b == Some(58235.0)));

        assert!(p.ingresar_dinero(38_998_761, 150.0).is_err_and(|e| e == ErrorTransaccion::UsuarioNoValidado));
        assert!(p.ingresar_dinero(41_192_387, -5.0).is_err_and(|e| e == ErrorTransaccion::MontoNegativo));
        assert!(p.ingresar_dinero(12_345_678, 999.0).is_err_and(|e| e == ErrorTransaccion::UsuarioInexistente));
    }

    #[test]
    fn test_retirar_dinero() {
        let mut p = crear_plataforma_base();
        assert!(p.validar_usuario(41_192_387).is_ok());
        assert!(p.validar_usuario(42_298_985).is_ok());
        assert!(p.validar_usuario(44_144_414).is_ok());

        assert!(p.ingresar_dinero(41_192_387, 1500.0).is_ok());

        assert!(p.retirar_dinero(41_192_387, 250.0, MedioPago::MercadoPago).is_ok_and(
            |t| t.validar_retiro_dinero(41_192_387, 250.0, MedioPago::MercadoPago) && 
                !t.validar_ingreso_dinero(41_192_387, 999.0)
        ));
        assert!(p.get_balance_usuario(41_192_387, Criptomoneda::DINERO_FIAT).is_ok_and(|b| b == Some(1500.0 - 250.0)));
        assert!(p.get_balance_usuario(41_192_387, Criptomoneda::BITCOIN).is_ok_and(|b| b == Some(0.0))); // comprobar que no haya afectado a otros balances

        assert!(p.retirar_dinero(41_192_387, 1000.0, MedioPago::Transferencia).is_ok_and(
            |t| t.validar_retiro_dinero(41_192_387, 1000.0, MedioPago::Transferencia)
        ));
        assert!(p.get_balance_usuario(41_192_387, Criptomoneda::DINERO_FIAT).is_ok_and(|b| b == Some(1500.0 - 250.0 - 1000.0)));

        assert!(p.retirar_dinero(38_998_761, 150.0, MedioPago::Transferencia).is_err_and(|e| e == ErrorTransaccion::UsuarioNoValidado));
        assert!(p.retirar_dinero(41_192_387, -5.0, MedioPago::Transferencia).is_err_and(|e| e == ErrorTransaccion::MontoNegativo));
        assert!(p.retirar_dinero(41_192_387, 300.0, MedioPago::Transferencia).is_err_and(|e| e == ErrorTransaccion::BalanceInsuficiente));
        assert!(p.retirar_dinero(44_144_414, 1.0, MedioPago::Transferencia).is_err_and(|e| e == ErrorTransaccion::BalanceInsuficiente));
        assert!(p.retirar_dinero(12_345_678, 1.0, MedioPago::Transferencia).is_err_and(|e| e == ErrorTransaccion::UsuarioInexistente));
    }

    #[test]
    fn test_comprar_cripto() {
        let mut p = crear_plataforma_base();
        assert!(p.validar_usuario(41_192_387).is_ok());
        assert!(p.validar_usuario(42_298_985).is_ok());
        assert!(p.validar_usuario(44_144_414).is_ok());

        assert!(p.ingresar_dinero(41_192_387, 1500.0).is_ok());

        let btc_coti = PlataformaXYZ::get_cotizacion(Criptomoneda::BITCOIN);
        assert!(p.comprar_cripto(41_192_387, 0.002, Criptomoneda::BITCOIN).is_ok_and(
            |t| t.validar_compra_cripto(41_192_387, 0.002, Criptomoneda::BITCOIN, btc_coti) && 
                !t.validar_venta_cripto(41_192_387, 999.0, "", 0.0)
        ));
        assert!(p.get_balance_usuario(41_192_387, Criptomoneda::DINERO_FIAT).is_ok_and(|b| b == Some(1500.0 - 0.002*btc_coti)));
        assert!(p.get_balance_usuario(41_192_387, Criptomoneda::BITCOIN).is_ok_and(|b| b == Some(0.002)));

        assert!(p.comprar_cripto(41_192_387, 0.015, Criptomoneda::BITCOIN).is_ok_and(
            |t| t.validar_compra_cripto(41_192_387, 0.015, Criptomoneda::BITCOIN, btc_coti)
        ));
        assert!(p.get_balance_usuario(41_192_387, Criptomoneda::DINERO_FIAT).is_ok_and(|b| b == Some(1500.0 - 0.002*btc_coti - 0.015*btc_coti)));
        assert!(p.get_balance_usuario(41_192_387, Criptomoneda::BITCOIN).is_ok_and(|b| b == Some(0.002+0.015)));

        assert!(p.comprar_cripto(38_998_761, 0.015, Criptomoneda::BITCOIN).is_err_and(|e| e == ErrorTransaccion::UsuarioNoValidado));
        assert!(p.comprar_cripto(41_192_387, -0.015, Criptomoneda::BITCOIN).is_err_and(|e| e == ErrorTransaccion::MontoNegativo));
        assert!(p.comprar_cripto(41_192_387, 1.0, Criptomoneda::BITCOIN).is_err_and(|e| e == ErrorTransaccion::BalanceInsuficiente));
        assert!(p.comprar_cripto(44_144_414, 0.00001, Criptomoneda::BITCOIN).is_err_and(|e| e == ErrorTransaccion::BalanceInsuficiente));
        assert!(p.comprar_cripto(12_345_678, 0.015, Criptomoneda::BITCOIN).is_err_and(|e| e == ErrorTransaccion::UsuarioInexistente));
        assert!(p.comprar_cripto(41_192_387, 0.5, "ABCD").is_err_and(|e| e == ErrorTransaccion::CriptoInexistente));
    }

    #[test]
    fn test_vender_cripto() {
        let mut p = crear_plataforma_base();
        assert!(p.validar_usuario(41_192_387).is_ok());
        assert!(p.validar_usuario(42_298_985).is_ok());
        assert!(p.validar_usuario(44_144_414).is_ok());

        assert!(p.ingresar_dinero(41_192_387, 1000.0).is_ok());
        assert!(p.comprar_cripto(41_192_387, 200000.0, Criptomoneda::SHIBAINU).is_ok());

        let shib_coti = PlataformaXYZ::get_cotizacion(Criptomoneda::SHIBAINU);
        assert!(p.vender_cripto(41_192_387, 100000.0, Criptomoneda::SHIBAINU).is_ok_and(
            |t| t.validar_venta_cripto(41_192_387, 100000.0, Criptomoneda::SHIBAINU, shib_coti) && 
                !t.validar_retiro_dinero(41_192_387, 999.0, MedioPago::MercadoPago)
        ));
        assert!(p.get_balance_usuario(41_192_387, Criptomoneda::DINERO_FIAT).is_ok_and(|b| b == Some(1000.0 - 200000.0*shib_coti + 100000.0*shib_coti)));
        assert!(p.get_balance_usuario(41_192_387, Criptomoneda::SHIBAINU).is_ok_and(|b| b == Some(200000.0 - 100000.0)));

       assert!(p.vender_cripto(41_192_387, 50000.0, Criptomoneda::SHIBAINU).is_ok_and(
            |t| t.validar_venta_cripto(41_192_387, 50000.0, Criptomoneda::SHIBAINU, shib_coti)
        ));
        assert!(p.get_balance_usuario(41_192_387, Criptomoneda::DINERO_FIAT).is_ok_and(|b| b == Some(1000.0 - 200000.0*shib_coti + 100000.0*shib_coti + 50000.0*shib_coti)));
        assert!(p.get_balance_usuario(41_192_387, Criptomoneda::SHIBAINU).is_ok_and(|b| b == Some(200000.0 - 100000.0 - 50000.0)));

        assert!(p.vender_cripto(38_998_761, 10000.0, Criptomoneda::SHIBAINU).is_err_and(|e| e == ErrorTransaccion::UsuarioNoValidado));
        assert!(p.vender_cripto(41_192_387, -10000.0, Criptomoneda::SHIBAINU).is_err_and(|e| e == ErrorTransaccion::MontoNegativo));
        assert!(p.vender_cripto(41_192_387, 150000.0, Criptomoneda::SHIBAINU).is_err_and(|e| e == ErrorTransaccion::BalanceInsuficiente));
        assert!(p.vender_cripto(44_144_414, 0.00001, Criptomoneda::SHIBAINU).is_err_and(|e| e == ErrorTransaccion::BalanceInsuficiente));
        assert!(p.vender_cripto(12_345_678, 10000.0, Criptomoneda::SHIBAINU).is_err_and(|e| e == ErrorTransaccion::UsuarioInexistente));
        assert!(p.vender_cripto(41_192_387, 0.5, "ABCD").is_err_and(|e| e == ErrorTransaccion::CriptoInexistente));
    }

    #[test]
    fn test_retirar_cripto() {
        let mut p = crear_plataforma_base();
        assert!(p.validar_usuario(41_192_387).is_ok());
        assert!(p.validar_usuario(42_298_985).is_ok());
        assert!(p.validar_usuario(44_144_414).is_ok());

        assert!(p.ingresar_dinero(41_192_387, 1000.0).is_ok());
        assert!(p.comprar_cripto(41_192_387, 500.0, Criptomoneda::TETHER).is_ok());

        let tether_coti = PlataformaXYZ::get_cotizacion(Criptomoneda::TETHER);
        let blockchain = Blockchain::ethereum();
        assert!(p.retirar_cripto(41_192_387, 250.0, Criptomoneda::TETHER, &blockchain).is_ok_and(
            |t| t.validar_retiro_cripto(41_192_387, 250.0, Criptomoneda::TETHER, tether_coti, &blockchain.prefijo, blockchain.get_hash()) && 
                !t.validar_recepcion_cripto(41_192_387, 999.0, "", 0.0, "")
        ));
        assert!(p.get_balance_usuario(41_192_387, Criptomoneda::TETHER).is_ok_and(|v| v == Some(500.0 - 250.0)));
        
        assert!(p.retirar_cripto(38_998_761, 250.0, Criptomoneda::TETHER, &blockchain).is_err_and(|e| e == ErrorTransaccion::UsuarioNoValidado));
        assert!(p.retirar_cripto(41_192_387, -1000.0, Criptomoneda::TETHER, &blockchain).is_err_and(|e| e == ErrorTransaccion::MontoNegativo));
        assert!(p.retirar_cripto(41_192_387, 1000.0, Criptomoneda::TETHER, &blockchain).is_err_and(|e| e == ErrorTransaccion::BalanceInsuficiente));
        assert!(p.retirar_cripto(44_144_414, 1.0, Criptomoneda::TETHER, &blockchain).is_err_and(|e| e == ErrorTransaccion::BalanceInsuficiente));
        assert!(p.retirar_cripto(44_144_414, 150.0, Criptomoneda::TETHER, &Blockchain::bitcoin()).is_err_and(|e| e == ErrorTransaccion::BlockchainInvalida));
        assert!(p.retirar_cripto(12_345_678, 250.0, Criptomoneda::TETHER, &blockchain).is_err_and(|e| e == ErrorTransaccion::UsuarioInexistente));
        assert!(p.retirar_cripto(41_192_387, 0.5, "ABCD", &blockchain).is_err_and(|e| e == ErrorTransaccion::CriptoInexistente));
    }

    #[test]
    fn test_recibir_cripto() {
        let mut p = crear_plataforma_base();
        assert!(p.validar_usuario(41_192_387).is_ok());
        assert!(p.validar_usuario(42_298_985).is_ok());
        assert!(p.validar_usuario(44_144_414).is_ok());

        assert!(p.ingresar_dinero(41_192_387, 1000.0).is_ok());
        assert!(p.comprar_cripto(41_192_387, 500.0, Criptomoneda::USDCOIN).is_ok());

        let usdc_coti: f64 = PlataformaXYZ::get_cotizacion(Criptomoneda::USDCOIN);
        let blockchain = Blockchain::ethereum();
        assert!(p.recibir_cripto(41_192_387, 250.0, Criptomoneda::USDCOIN, &blockchain).is_ok_and(
            |t| t.validar_recepcion_cripto(41_192_387, 250.0, Criptomoneda::USDCOIN, usdc_coti, &blockchain.prefijo) && 
                !t.validar_retiro_cripto(41_192_387, 999.0, "", 0.0, "", "".to_string())
        ));
        assert!(p.get_balance_usuario(41_192_387, Criptomoneda::USDCOIN).is_ok_and(|v| v == Some(500.0 + 250.0)));

        assert!(p.recibir_cripto(41_192_387, 150.0, Criptomoneda::USDCOIN, &blockchain).is_ok_and(
            |t| t.validar_recepcion_cripto(41_192_387, 150.0, Criptomoneda::USDCOIN, usdc_coti, &blockchain.prefijo)
        ));
        assert!(p.get_balance_usuario(41_192_387, Criptomoneda::USDCOIN).is_ok_and(|v| v == Some(500.0 + 250.0 + 150.0)));
        
        assert!(p.recibir_cripto(38_998_761, 250.0, Criptomoneda::USDCOIN, &blockchain).is_err_and(|e| e == ErrorTransaccion::UsuarioNoValidado));
        assert!(p.recibir_cripto(41_192_387, -1000.0, Criptomoneda::USDCOIN, &blockchain).is_err_and(|e| e == ErrorTransaccion::MontoNegativo));
        assert!(p.recibir_cripto(44_144_414, 150.0, Criptomoneda::USDCOIN, &Blockchain::bitcoin()).is_err_and(|e| e == ErrorTransaccion::BlockchainInvalida));
        assert!(p.recibir_cripto(12_345_678, 250.0, Criptomoneda::USDCOIN, &blockchain).is_err_and(|e| e == ErrorTransaccion::UsuarioInexistente));
        assert!(p.recibir_cripto(41_192_387, 0.5, "ABCD", &blockchain).is_err_and(|e| e == ErrorTransaccion::CriptoInexistente));
    }

    #[test]
    fn test_cripto_mas_vendida() {
        let mut p = crear_plataforma_base_compras_ventas();
        assert_eq!(p.cripto_mas_vendida(), Criptomoneda::shibainu());

        assert!(p.comprar_cripto(41_192_387, 0.1, Criptomoneda::TETHER).is_ok());
        for _ in 0..4 {
            assert!(p.vender_cripto(41_192_387, 0.001, Criptomoneda::TETHER).is_ok());
        }
        assert_eq!(p.cripto_mas_vendida(), Criptomoneda::tether());

        assert!(p.comprar_cripto(41_192_387, 1.0, Criptomoneda::USDCOIN).is_ok());
        for _ in 0..7 {
            assert!(p.vender_cripto(41_192_387, 0.001, Criptomoneda::USDCOIN).is_ok());
        }
        assert_eq!(p.cripto_mas_vendida(), Criptomoneda::usdcoin());
    }

    #[test]
    fn test_cripto_mas_comprada() {
        let mut p = crear_plataforma_base_compras_ventas();
        assert_eq!(p.cripto_mas_comprada(), Criptomoneda::ethereum());

        for _ in 0..4 {
            assert!(p.comprar_cripto(41_192_387, 0.1, Criptomoneda::TETHER).is_ok());
        }
        assert_eq!(p.cripto_mas_comprada(), Criptomoneda::tether());

        for _ in 0..7 {
            assert!(p.comprar_cripto(41_192_387, 1.0, Criptomoneda::USDCOIN).is_ok());
        }
        assert_eq!(p.cripto_mas_comprada(), Criptomoneda::usdcoin());
    }

    #[test]
    fn test_cripto_mas_volumen_ventas() {
        let mut p = crear_plataforma_base_compras_ventas();
        assert_eq!(p.cripto_mas_volumen_ventas(), Criptomoneda::ethereum());

        assert!(p.comprar_cripto(41_192_387, 1_000_000.0, Criptomoneda::TETHER).is_ok());
        assert!(p.vender_cripto(41_192_387, 1_000_000.0, Criptomoneda::TETHER).is_ok());
        assert_eq!(p.cripto_mas_volumen_ventas(), Criptomoneda::tether());

        assert!(p.comprar_cripto(41_192_387, 2_000_000.0, Criptomoneda::USDCOIN).is_ok());
        assert!(p.vender_cripto(41_192_387, 2_000_000.0, Criptomoneda::USDCOIN).is_ok());
        assert_eq!(p.cripto_mas_volumen_ventas(), Criptomoneda::usdcoin());
    }

    #[test]
    fn test_cripto_mas_volumen_compras() {
        let mut p = crear_plataforma_base_compras_ventas();
        assert_eq!(p.cripto_mas_volumen_compras(), Criptomoneda::bitcoin());

        assert!(p.comprar_cripto(41_192_387, 1_000_000.0, Criptomoneda::TETHER).is_ok());
        assert_eq!(p.cripto_mas_volumen_compras(), Criptomoneda::tether());

        assert!(p.comprar_cripto(41_192_387, 2_000_000.0, Criptomoneda::USDCOIN).is_ok());
        assert_eq!(p.cripto_mas_volumen_compras(), Criptomoneda::usdcoin());
    }
}
