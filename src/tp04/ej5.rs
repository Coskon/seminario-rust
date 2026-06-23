#![allow(unused)]
use std::collections::{BTreeMap, HashMap, btree_map::Entry};
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
pub struct Rng {
    state: u64
}

impl Rng {
    const TEST_SEED: u64 = 1234_u64;

    pub fn new(seed: u64) -> Self {
        Rng { state: seed.saturating_add(1) } // evita seed=0
    }

    pub fn test_default() -> Self {
        Rng::new(Rng::TEST_SEED)
    }

    pub fn get_next(&mut self) -> u64 {
        // algoritmo: xorshift64*
        self.state ^= self.state >> 12;
        self.state ^= self.state << 25;
        self.state ^= self.state >> 27;
        self.state.wrapping_mul(2685821657736338717_u64)
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Blockchain {
    pub nombre: String,
    pub prefijo: String
}

impl Blockchain {
    pub fn get_hash(&self, rng: &mut Rng) -> String {
        let num = rng.get_next();
        self.nombre.clone() + &num.to_string()
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
pub enum Criptomoneda {
    Bitcoin, Tether, Ethereum, USDCoin, ShibaInu
}

impl Criptomoneda {
    pub fn prefijo(&self) -> String {
        match self {
            Criptomoneda::Bitcoin => "BTC".to_string(),
            Criptomoneda::Tether => "USDT".to_string(),
            Criptomoneda::Ethereum => "ETH".to_string(),
            Criptomoneda::USDCoin => "USDC".to_string(),
            Criptomoneda::ShibaInu => "SHIB".to_string(),
        }
    }

    pub fn lista_prefijos() -> [String; 5] {
        [Criptomoneda::Bitcoin.prefijo(), Criptomoneda::Tether.prefijo(), Criptomoneda::Ethereum.prefijo(), Criptomoneda::USDCoin.prefijo(), Criptomoneda::ShibaInu.prefijo()]
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct DatosCriptomoneda {
    pub nombre: String,
    pub prefijo: String,
    blockchains: Vec<Blockchain>
}

impl DatosCriptomoneda {
    pub fn from_cripto(cripto: &Criptomoneda) -> Self {
        match cripto {
            Criptomoneda::Bitcoin => DatosCriptomoneda::bitcoin(),
            Criptomoneda::Tether => DatosCriptomoneda::tether(),
            Criptomoneda::Ethereum => DatosCriptomoneda::ethereum(),
            Criptomoneda::USDCoin => DatosCriptomoneda::usdcoin(),
            Criptomoneda::ShibaInu => DatosCriptomoneda::shibainu()
        }
    }

    pub fn bitcoin() -> Self {
        DatosCriptomoneda { nombre: "Bitcoin".to_string(), prefijo: Criptomoneda::Bitcoin.prefijo(), blockchains: vec![Blockchain::bitcoin()]}
    }

    pub fn tether() -> Self {
        DatosCriptomoneda { nombre: "Tether".to_string(), prefijo: Criptomoneda::Tether.prefijo(), blockchains: vec![Blockchain::ethereum(), Blockchain::tron(), Blockchain::solana(), Blockchain::polygon()]}
    }

    pub fn ethereum() -> Self {
        DatosCriptomoneda { nombre: "Ethereum".to_string(), prefijo: Criptomoneda::Ethereum.prefijo(), blockchains: vec![Blockchain::ethereum()]}
    }

    pub fn usdcoin() -> Self {
        DatosCriptomoneda { nombre: "USD Coin".to_string(), prefijo: Criptomoneda::USDCoin.prefijo(), blockchains: vec![Blockchain::ethereum(), Blockchain::solana(), Blockchain::polygon()]}
    }

    pub fn shibainu() -> Self {
        DatosCriptomoneda { nombre: "Shiba Inu".to_string(), prefijo: Criptomoneda::ShibaInu.prefijo(), blockchains: vec![Blockchain::ethereum(), Blockchain::polygon()]}
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Balance {
    map: HashMap<String, f64> // prefijo cripto/fiat -- balance
}

impl Balance {
    pub fn default() -> Self {
        let mut m = HashMap::from(Criptomoneda::lista_prefijos().map(|p| (p, 0.0)));
        m.insert("DINERO_FIAT".to_string(), 0.0);
        Balance { map: m } 
    }

    pub fn tiene_suficiente_dinero(&self, monto: f64) -> bool {
        *self.map.get("DINERO_FIAT").expect("Balance no tiene entry DINERO_FIAT") >= monto
    }

    pub fn get_dinero(&self) -> f64 {
        self.map.get("DINERO_FIAT").copied().expect("Balance no tiene entry DINERO_FIAT")
    }

    pub fn agregar_dinero(&mut self, monto: f64) {
        *self.map.get_mut("DINERO_FIAT").expect("Balance no tiene entry DINERO_FIAT") += monto;
    }

    pub fn quitar_dinero(&mut self, monto: f64) {
        *self.map.get_mut("DINERO_FIAT").expect("Balance no tiene entry DINERO_FIAT") -= monto;
    }

    pub fn tiene_suficiente_cripto(&self, cripto: &Criptomoneda, monto: f64) -> bool {
        *self.map.get(&cripto.prefijo()).expect("Balance no tiene entry de la cripto") >= monto
    }

    pub fn get_cripto(&self, cripto: &Criptomoneda) -> f64 {
        self.map.get(&cripto.prefijo()).copied().expect("Balance no tiene entry de la cripto")
    }

    pub fn agregar_cripto(&mut self, cripto: &Criptomoneda, monto: f64) {
        *self.map.entry(cripto.prefijo()).or_insert(0.0) += monto;
    }

    pub fn quitar_cripto(&mut self, cripto: &Criptomoneda, monto: f64) {
        *self.map.entry(cripto.prefijo()).or_insert(0.0) -= monto;
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
        criptomoneda: Criptomoneda,
        monto: f64,
        usuario: Usuario,
        cotizacion: f64
    }, 
    VentaCripto {
        fecha: Fecha,
        criptomoneda: Criptomoneda,
        monto: f64,
        usuario: Usuario,
        cotizacion: f64
    }, 
    RetiroCripto {
        fecha: Fecha,
        criptomoneda: Criptomoneda,
        monto: f64,
        usuario: Usuario,
        cotizacion: f64,
        blockchain: Blockchain,
        hash: String
    },
    RecepcionCripto {
        fecha: Fecha,
        criptomoneda: Criptomoneda,
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

    pub fn validar_compra_cripto(&self, dni_validar: u32, monto_validar: f64, cripto_validar: Criptomoneda, cotizacion_validar: f64) -> bool {
        match self {
            Transaccion::CompraCripto { monto, usuario, criptomoneda, cotizacion, .. } => 
                *monto == monto_validar && usuario.dni == dni_validar && *criptomoneda == cripto_validar && *cotizacion == cotizacion_validar,
            _ => false
        }
    }

    pub fn validar_venta_cripto(&self, dni_validar: u32, monto_validar: f64, cripto_validar: Criptomoneda, cotizacion_validar: f64) -> bool {
        match self {
            Transaccion::VentaCripto { monto, usuario, criptomoneda, cotizacion, .. } => 
                *monto == monto_validar && usuario.dni == dni_validar && *criptomoneda == cripto_validar && *cotizacion == cotizacion_validar,
            _ => false
        }
    }

    pub fn validar_retiro_cripto(&self, dni_validar: u32, monto_validar: f64, cripto_validar: Criptomoneda, cotizacion_validar: f64, blockchain_validar: &str, hash_validar: &str) -> bool {
        match self {
            Transaccion::RetiroCripto { monto, usuario, criptomoneda, cotizacion, hash, blockchain,  .. } => 
                *monto == monto_validar && usuario.dni == dni_validar && *criptomoneda == cripto_validar && *cotizacion == cotizacion_validar && *hash == hash_validar && blockchain.prefijo == blockchain_validar,
            _ => false
        }
    }

    pub fn validar_recepcion_cripto(&self, dni_validar: u32, monto_validar: f64, cripto_validar: Criptomoneda, cotizacion_validar: f64, blockchain_validar: &str) -> bool {
        match self {
            Transaccion::RecepcionCripto { monto, usuario, criptomoneda, cotizacion, blockchain, .. } => 
                *monto == monto_validar && usuario.dni == dni_validar && *criptomoneda == cripto_validar && *cotizacion == cotizacion_validar && blockchain.prefijo == blockchain_validar,
            _ => false
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum ErrorTransaccion {
    UsuarioInexistente, MontoInvalido, BalanceInsuficiente, UsuarioNoValidado, BlockchainInvalida
}

#[derive(PartialEq, Debug, Clone)]
pub enum ErrorUsuario {
    UsuarioYaExiste, UsuarioInexistente
}

pub struct PlataformaXYZ {
    usuarios: BTreeMap<u32, Usuario>, // dni -- usuario
    transacciones: BTreeMap<u32, Vec<Transaccion>>, // dni -- historial de transacciones del usuario
    balances: BTreeMap<u32, Balance>, // dni -- balance de las distintas criptos del usuario
    rng: Rng
}

impl PlataformaXYZ {
    pub fn new(seed: u64) -> Self {
        PlataformaXYZ { usuarios: BTreeMap::new(), balances: BTreeMap::new(), transacciones: BTreeMap::new(), rng: Rng::new(seed) }
    }

    pub fn get_cotizacion(cripto: &Criptomoneda) -> f64 {
        // devuelve el valor (en dolares) de 1 unidad de la criptomoneda
        match cripto {
            Criptomoneda::Bitcoin => 63030.62,
            Criptomoneda::Tether => 1.0,
            Criptomoneda::Ethereum => 1700.99,
            Criptomoneda::USDCoin => 1.0,
            Criptomoneda::ShibaInu => 0.000004735
        }
    }

    pub fn registrar_usuario(&mut self, usuario: Usuario) -> Result<(), ErrorUsuario> {
        let dni = usuario.dni;
        match self.usuarios.entry(dni) {
            Entry::Occupied(v) => Err(ErrorUsuario::UsuarioYaExiste),
            Entry::Vacant(v) => {
                v.insert(usuario);
                self.transacciones.insert(dni, vec![]);
                self.balances.insert(dni, Balance::default());
                Ok(())
            }
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
            Ok(user)
        } else {
            Err(ErrorUsuario::UsuarioInexistente)
        }
    }

    pub fn get_balance_cripto_usuario(&self, dni: u32, cripto: Criptomoneda) -> Result<f64, ErrorUsuario> {
        if let Some(balance) = self.balances.get(&dni) {
            Ok(balance.get_cripto(&cripto))
        } else {
            Err(ErrorUsuario::UsuarioInexistente)
        }
    }

    pub fn get_balance_usuario(&self, dni: u32) -> Result<f64, ErrorUsuario> {
        if let Some(balance) = self.balances.get(&dni) {
            Ok(balance.get_dinero())
        } else {
            Err(ErrorUsuario::UsuarioInexistente)
        }
    }

    pub fn ingresar_dinero(&mut self, dni: u32, monto: f64) -> Result<Transaccion, ErrorTransaccion> {
        if !monto.is_finite() || monto <= 0.0 {
            Err(ErrorTransaccion::MontoInvalido)
        } else if let Some(user) = self.usuarios.get(&dni) {
            if !user.identidad_validada() {
                return Err(ErrorTransaccion::UsuarioNoValidado);
            }

            let balance = self.balances.get_mut(&dni).expect("Usuario no tiene balances");
            balance.agregar_dinero(monto);

            let t = Transaccion::IngresoDinero {
                fecha: Fecha::fecha_actual(),
                monto,
                usuario: user.clone()
            };
            // si existe el usuario en self.usuarios, tambien debe existir en self.transacciones
            // ya que se crean al mismo tiempo en registrar_usuario
            self.transacciones.get_mut(&dni).expect("Usuario no tiene transacciones").push(t.clone());
            Ok(t)
        } else {
            Err(ErrorTransaccion::UsuarioInexistente)
        }
    }

    pub fn retirar_dinero(&mut self, dni: u32, monto: f64, medio: MedioPago) -> Result<Transaccion, ErrorTransaccion> {
        if !monto.is_finite() || monto <= 0.0 {
            Err(ErrorTransaccion::MontoInvalido)
        } else if let Some(user) = self.usuarios.get(&dni) {
            if !user.identidad_validada() {
                return Err(ErrorTransaccion::UsuarioNoValidado);
            }
            
            let balance = self.balances.get_mut(&dni).expect("Usuario no tiene balances");
            if !balance.tiene_suficiente_dinero(monto) {
                return Err(ErrorTransaccion::BalanceInsuficiente);
            }
            balance.quitar_dinero(monto);

            let t = Transaccion::RetiroDinero {
                fecha: Fecha::fecha_actual(),
                monto,
                usuario: user.clone(),
                medio
            };
            self.transacciones.get_mut(&dni).expect("Usuario no tiene transacciones").push(t.clone());
            Ok(t)
        } else {
            Err(ErrorTransaccion::UsuarioInexistente)
        }
    }

    pub fn comprar_cripto(&mut self, dni: u32, cantidad: f64, cripto: Criptomoneda) -> Result<Transaccion, ErrorTransaccion> {
        if !cantidad.is_finite() || cantidad <= 0.0 {
            Err(ErrorTransaccion::MontoInvalido)
        } else if let Some(user) = self.usuarios.get(&dni) {
            if !user.identidad_validada() {
                return Err(ErrorTransaccion::UsuarioNoValidado);
            }
            
            let cotizacion = PlataformaXYZ::get_cotizacion(&cripto);
            let balance = self.balances.get_mut(&dni).expect("Usuario no tiene balances");
            let monto_a_pagar = cantidad*cotizacion;
            if !balance.tiene_suficiente_dinero(monto_a_pagar) {
                return Err(ErrorTransaccion::BalanceInsuficiente);
            }
            balance.quitar_dinero(monto_a_pagar);
            balance.agregar_cripto(&cripto, cantidad);
            
            let t = Transaccion::CompraCripto { 
                fecha: Fecha::fecha_actual(),
                criptomoneda: cripto, 
                monto: cantidad, 
                usuario: user.clone(), 
                cotizacion 
            };
            self.transacciones.get_mut(&dni).expect("Usuario no tiene transacciones").push(t.clone());
            Ok(t)
        } else {
            Err(ErrorTransaccion::UsuarioInexistente)
        }
    }

    pub fn vender_cripto(&mut self, dni: u32, cantidad: f64, cripto: Criptomoneda) -> Result<Transaccion, ErrorTransaccion> {
        if !cantidad.is_finite() || cantidad <= 0.0 {
            Err(ErrorTransaccion::MontoInvalido)
        } else if let Some(user) = self.usuarios.get(&dni) {
            if !user.identidad_validada() {
                return Err(ErrorTransaccion::UsuarioNoValidado);
            }
            
            let cotizacion = PlataformaXYZ::get_cotizacion(&cripto);
            let balance = self.balances.get_mut(&dni).expect("Usuario no tiene balances");
            if !balance.tiene_suficiente_cripto(&cripto, cantidad) {
                return Err(ErrorTransaccion::BalanceInsuficiente);
            }
            balance.quitar_cripto(&cripto, cantidad);
            balance.agregar_dinero(cantidad*cotizacion);
            
            let t = Transaccion::VentaCripto { 
                fecha: Fecha::fecha_actual(),
                criptomoneda: cripto, 
                monto: cantidad, 
                usuario: user.clone(), 
                cotizacion 
            };
            self.transacciones.get_mut(&dni).expect("Usuario no tiene transacciones").push(t.clone());
            Ok(t)
        } else {
            Err(ErrorTransaccion::UsuarioInexistente)
        }
    }

    pub fn retirar_cripto(&mut self, dni: u32, cantidad: f64, cripto: Criptomoneda, blockchain: &Blockchain) -> Result<Transaccion, ErrorTransaccion> {
        if !cantidad.is_finite() || cantidad <= 0.0 {
            Err(ErrorTransaccion::MontoInvalido)
        } else if let Some(user) = self.usuarios.get(&dni) {
            if !user.identidad_validada() {
                return Err(ErrorTransaccion::UsuarioNoValidado);
            }

            let cm = DatosCriptomoneda::from_cripto(&cripto);
            if !cm.blockchains.contains(blockchain) {
                return Err(ErrorTransaccion::BlockchainInvalida);
            }
            
            let cotizacion = PlataformaXYZ::get_cotizacion(&cripto);
            let balance = self.balances.get_mut(&dni).expect("Usuario no tiene balances");
            if !balance.tiene_suficiente_cripto(&cripto, cantidad) {
                return Err(ErrorTransaccion::BalanceInsuficiente);
            }
            balance.quitar_cripto(&cripto, cantidad);
            
            let t = Transaccion::RetiroCripto { 
                fecha: Fecha::fecha_actual(),
                criptomoneda: cripto, 
                monto: cantidad, 
                usuario: user.clone(), 
                cotizacion,
                blockchain: blockchain.clone(),
                hash: blockchain.get_hash(&mut self.rng)
            };
            self.transacciones.get_mut(&dni).expect("Usuario no tiene transacciones").push(t.clone());
            Ok(t)
        } else {
            Err(ErrorTransaccion::UsuarioInexistente)
        }
    }

    pub fn recibir_cripto(&mut self, dni: u32, cantidad: f64, cripto: Criptomoneda, blockchain: &Blockchain) -> Result<Transaccion, ErrorTransaccion> {
        if !cantidad.is_finite() || cantidad <= 0.0 {
            Err(ErrorTransaccion::MontoInvalido)
        } else if let Some(user) = self.usuarios.get(&dni) {
            if !user.identidad_validada() {
                return Err(ErrorTransaccion::UsuarioNoValidado);
            }

            let cm = DatosCriptomoneda::from_cripto(&cripto);
            if !cm.blockchains.contains(blockchain) {
                return Err(ErrorTransaccion::BlockchainInvalida);
            }
            
            let cotizacion = PlataformaXYZ::get_cotizacion(&cripto);
            let balance = self.balances.get_mut(&dni).expect("Usuario no tiene balances");
            balance.agregar_cripto(&cripto, cantidad);
            
            let t = Transaccion::RecepcionCripto { 
                fecha: Fecha::fecha_actual(),
                criptomoneda: cripto, 
                monto: cantidad, 
                usuario: user.clone(), 
                cotizacion,
                blockchain: blockchain.clone()
            };
            self.transacciones.get_mut(&dni).expect("Usuario no tiene transacciones").push(t.clone());
            Ok(t)
        } else {
            Err(ErrorTransaccion::UsuarioInexistente)
        }
    }

    fn incrementar_cripto_arr<T: std::ops::AddAssign>(cripto: &Criptomoneda, arr: &mut [T; 5], val: T) {
        match cripto {
            Criptomoneda::Bitcoin => arr[0] += val,
            Criptomoneda::Tether => arr[1] += val,
            Criptomoneda::Ethereum => arr[2] += val,
            Criptomoneda::USDCoin => arr[3] += val,
            Criptomoneda::ShibaInu => arr[4] += val
        };
    }

    fn get_max_cripto_from_arr<T: PartialOrd>(&self, arr: &[T; 5]) -> DatosCriptomoneda {
        let maxpos = arr.iter().enumerate().max_by(|(i1, v1), (i2, v2)| v1.partial_cmp(v2).expect("No se pudo ordenar valores")).expect("No se encontro valor maximo").0;
        let cripto = match maxpos {
            0 => Criptomoneda::Bitcoin,
            1 => Criptomoneda::Tether,
            2 => Criptomoneda::Ethereum,
            3 => Criptomoneda::USDCoin,
            4 => Criptomoneda::ShibaInu,
            v => unreachable!("Valor inesperado: {v}")
        };
        DatosCriptomoneda::from_cripto(&cripto)
    }

    pub fn cripto_mas_vendida(&self) -> DatosCriptomoneda {
        let mut cant_ventas_cripto = [0_usize; 5];
        self.transacciones.values().flatten().for_each(|t| 
            if let Transaccion::VentaCripto { criptomoneda, .. } = t {
                PlataformaXYZ::incrementar_cripto_arr(criptomoneda, &mut cant_ventas_cripto, 1);
            }
        );
        self.get_max_cripto_from_arr(&cant_ventas_cripto)
    }

    pub fn cripto_mas_comprada(&self) -> DatosCriptomoneda {
        let mut cant_compras_cripto = [0_usize; 5];
        self.transacciones.values().flatten().for_each(|t| 
            if let Transaccion::CompraCripto { criptomoneda, .. } = t {
                PlataformaXYZ::incrementar_cripto_arr(criptomoneda, &mut cant_compras_cripto, 1);
            }
        );
        self.get_max_cripto_from_arr(&cant_compras_cripto)
    }

    pub fn cripto_mas_volumen_ventas(&self) -> DatosCriptomoneda {
        let mut volumen_ventas_cripto = [0.0; 5];
        self.transacciones.values().flatten().for_each(|t| 
            if let Transaccion::VentaCripto { criptomoneda, monto, cotizacion, .. } = t {
                PlataformaXYZ::incrementar_cripto_arr(criptomoneda, &mut volumen_ventas_cripto, monto*cotizacion);
            }
        );
        self.get_max_cripto_from_arr(&volumen_ventas_cripto)
    }

    pub fn cripto_mas_volumen_compras(&self) -> DatosCriptomoneda {
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
    use super::*;

    #[test]
    fn rng() {
        let mut rng = Rng::test_default();
        let mut rng2 = Rng::new(Rng::TEST_SEED - 0x10); // seed distinta pero relativamente cercana
        let expected_nums = [
            304805589531039275,
            2147874104627985986,
            16155265163649813729,
            4403088556832279230,
            10376954439805897369,
            93720924362322587,
            13446763960826623206,
            12955923286034900152,
            6178981222045246712,
            3772662766451115367
        ];
        for expected in expected_nums {
            let num = rng.get_next();
            assert_eq!(num, expected);
            assert_ne!(num, rng2.get_next()); // deberia producir distintos numeros (muy baja probabilidad de que sean iguales)
        }

        let mut rng = Rng::new(Rng::TEST_SEED - 0x10); // misma seed que rng2 pero distinto estado (rng2 ya produjo 10 numeros)
        let expected_nums = [
            11066679356555752327,
            2355764329262227875,
            16737063961363423699,
            2660795052393371886,
            12503910303823595905,
            15784862469218972292,
            16827450051425673798,
            4752826200790062997,
            6752449077110964532,
            1158291332045498429
        ];
        for expected in expected_nums {
            let num = rng.get_next();
            assert_eq!(num, expected);
            assert_ne!(num, rng2.get_next());
        }
    }

    #[test]
    fn mismo_blockchain_hash() {
        let mut rng = Rng::test_default();
        let b = Blockchain::bitcoin();
        assert_eq!(b.get_hash(&mut rng), b.nombre.clone() + &304805589531039275_u64.to_string());
        assert_eq!(b.get_hash(&mut rng), b.nombre.clone() + &2147874104627985986_u64.to_string());
        assert_eq!(b.get_hash(&mut rng), b.nombre.clone() + &16155265163649813729_u64.to_string());
        assert_eq!(b.get_hash(&mut rng), b.nombre.clone() + &4403088556832279230_u64.to_string());
    }

    #[test]
    fn distintos_blockchain_hash() {
        let mut rng = Rng::test_default();
        
        let b = Blockchain::bitcoin();
        assert_eq!(b.get_hash(&mut rng), b.nombre.clone() + &304805589531039275_u64.to_string());

        let b = Blockchain::tron();
        assert_eq!(b.get_hash(&mut rng), b.nombre.clone() + &2147874104627985986_u64.to_string());

        let b = Blockchain::solana();
        assert_eq!(b.get_hash(&mut rng), b.nombre.clone() + &16155265163649813729_u64.to_string());
        
        let b = Blockchain::ethereum();
        assert_eq!(b.get_hash(&mut rng), b.nombre.clone() + &4403088556832279230_u64.to_string());
        
        let b = Blockchain::polygon();
        assert_eq!(b.get_hash(&mut rng), b.nombre.clone() + &10376954439805897369_u64.to_string());
    }
    
    #[test]
    fn registrar_usuario() {
        let mut p = PlataformaXYZ::new(Rng::TEST_SEED);
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

        assert!(p.get_balance_usuario(41_192_387).is_ok_and(|b| b == 0.0));
        assert!(p.get_balance_cripto_usuario(41_192_387, Criptomoneda::Bitcoin).is_ok_and(|b| b == 0.0));
        assert!(p.get_balance_usuario(12_345_678).is_err_and(|e| e == ErrorUsuario::UsuarioInexistente));
    }

    fn crear_plataforma_base() -> PlataformaXYZ {
        let mut p = PlataformaXYZ::new(Rng::TEST_SEED);
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
        assert!(p.comprar_cripto(41_192_387, 4.0, Criptomoneda::ShibaInu).is_ok());
        assert!(p.comprar_cripto(41_192_387, 10.0, Criptomoneda::Ethereum).is_ok());
        assert!(p.comprar_cripto(41_192_387, 25.0, Criptomoneda::Ethereum).is_ok());
        assert!(p.comprar_cripto(41_192_387, 1.0, Criptomoneda::Bitcoin).is_ok());

        assert!(p.vender_cripto(41_192_387, 1.0, Criptomoneda::ShibaInu).is_ok());
        assert!(p.vender_cripto(41_192_387, 1.0, Criptomoneda::ShibaInu).is_ok());
        assert!(p.vender_cripto(41_192_387, 1.0, Criptomoneda::ShibaInu).is_ok());
        assert!(p.vender_cripto(41_192_387, 35.0, Criptomoneda::Ethereum).is_ok());
        assert!(p.vender_cripto(41_192_387, 0.2, Criptomoneda::Bitcoin).is_ok());
        assert!(p.vender_cripto(41_192_387, 0.6, Criptomoneda::Bitcoin).is_ok());

        /*
        cotizaciones:
            Criptomoneda::Bitcoin  => 63030.62
            Criptomoneda::Tether   => 1.0
            Criptomoneda::Ethereum => 1700.99
            Criptomoneda::USDCoin  => 1.0
            Criptomoneda::ShibaInu => 0.000004735

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
    fn validar_usuario_normal() {
        let mut p = crear_plataforma_base();
        assert!(p.get_usuario(41_192_387).is_ok_and(|u| !u.identidad_validada()));
        assert!(p.validar_usuario(41_192_387).is_ok() && p.get_usuario(41_192_387).is_ok_and(|u| u.identidad_validada()));
    
        assert!(p.get_usuario(44_144_414).is_ok_and(|u| !u.identidad_validada()));
        assert!(p.validar_usuario(44_144_414).is_ok() && p.get_usuario(44_144_414).is_ok_and(|u| u.identidad_validada()));
    }

    #[test]
    fn validar_usuario_inexistente() {
        let mut p = PlataformaXYZ::new(Rng::TEST_SEED);
        assert!(p.validar_usuario(12_345_678).is_err_and(|e| e == ErrorUsuario::UsuarioInexistente));
        assert!(p.validar_usuario(41_192_387).is_err_and(|e| e == ErrorUsuario::UsuarioInexistente));
    }

    #[test]
    fn ingresar_dinero_normal() {
        let mut p = crear_plataforma_base();
        assert!(p.validar_usuario(41_192_387).is_ok());
        assert!(p.validar_usuario(44_144_414).is_ok());

        assert!(p.ingresar_dinero(41_192_387, 999.0).is_ok_and(
            |t| t.validar_ingreso_dinero(41_192_387, 999.0) && 
                !t.validar_compra_cripto(41_192_387, 999.0, Criptomoneda::Bitcoin, 0.0)
        ));
        assert!(p.get_balance_usuario(41_192_387).is_ok_and(|b| b == 999.0));
        assert!(p.get_balance_cripto_usuario(41_192_387, Criptomoneda::Bitcoin).is_ok_and(|b| b == 0.0)); // comprobar que no haya afectado a otros balances

        assert!(p.ingresar_dinero(41_192_387, 150.0).is_ok_and(
            |t| t.validar_ingreso_dinero(41_192_387, 150.0)
        ));
        assert!(p.get_balance_usuario(41_192_387).is_ok_and(|b| b == 999.0 + 150.0));

        assert!(p.ingresar_dinero(41_192_387, 0.0_f64.next_up()).is_ok_and(
            |t| t.validar_ingreso_dinero(41_192_387, 0.0_f64.next_up())
        ));
        assert!(p.get_balance_usuario(41_192_387).is_ok_and(|b| b == 999.0 + 150.0 + 0.0_f64.next_up()));

        assert!(p.ingresar_dinero(44_144_414, f64::MAX).is_ok_and(
            |t| t.validar_ingreso_dinero(44_144_414, f64::MAX)
        ));
        assert!(p.get_balance_usuario(44_144_414).is_ok_and(|b| b == f64::MAX));
    }

    #[test]
    fn ingresar_dinero_monto_invalido() {
        let mut p = crear_plataforma_base();
        assert!(p.validar_usuario(41_192_387).is_ok());

        assert!(p.ingresar_dinero(41_192_387, -5.0).is_err_and(|e| e == ErrorTransaccion::MontoInvalido));
        assert!(p.ingresar_dinero(41_192_387, 0.0).is_err_and(|e| e == ErrorTransaccion::MontoInvalido));
        assert!(p.ingresar_dinero(41_192_387, 0.0_f64.next_down()).is_err_and(|e| e == ErrorTransaccion::MontoInvalido));
        assert!(p.ingresar_dinero(41_192_387, f64::MIN).is_err_and(|e| e == ErrorTransaccion::MontoInvalido));
        assert!(p.ingresar_dinero(41_192_387, f64::INFINITY).is_err_and(|e| e == ErrorTransaccion::MontoInvalido));
        assert!(p.ingresar_dinero(41_192_387, f64::NEG_INFINITY).is_err_and(|e| e == ErrorTransaccion::MontoInvalido));
        assert!(p.ingresar_dinero(41_192_387, f64::NAN).is_err_and(|e| e == ErrorTransaccion::MontoInvalido));
    
        // comprobar que no se haya realizado ninguna transaccion
        assert!(p.get_balance_usuario(41_192_387).is_ok_and(|b| b == 0.0));
    }

    #[test]
    fn ingresar_dinero_error_usuarios() {
        let mut p = crear_plataforma_base();
        assert!(p.ingresar_dinero(41_192_387, 150.0).is_err_and(|e| e == ErrorTransaccion::UsuarioNoValidado));
        assert!(p.ingresar_dinero(12_345_678, 999.0).is_err_and(|e| e == ErrorTransaccion::UsuarioInexistente));
    }

    #[test]
    fn retirar_dinero_normal() {
        let mut p = crear_plataforma_base();
        assert!(p.validar_usuario(41_192_387).is_ok());
        assert!(p.validar_usuario(44_144_414).is_ok());

        assert!(p.ingresar_dinero(41_192_387, 1500.0).is_ok());

        assert!(p.retirar_dinero(41_192_387, 250.0, MedioPago::MercadoPago).is_ok_and(
            |t| t.validar_retiro_dinero(41_192_387, 250.0, MedioPago::MercadoPago) && 
                !t.validar_ingreso_dinero(41_192_387, 250.0)
        ));
        assert!(p.get_balance_usuario(41_192_387).is_ok_and(|b| b == 1500.0 - 250.0));
        
        assert!(p.retirar_dinero(41_192_387, 0.0_f64.next_up(), MedioPago::MercadoPago).is_ok_and(
            |t| t.validar_retiro_dinero(41_192_387, 0.0_f64.next_up(), MedioPago::MercadoPago) && 
                !t.validar_ingreso_dinero(41_192_387, 0.0_f64.next_up())
        ));
        assert!(p.get_balance_usuario(41_192_387).is_ok_and(|b| b == 1500.0 - 250.0 - 0.0_f64.next_up()));
    }

    #[test]
    fn retirar_dinero_monto_invalido() {
        let mut p = crear_plataforma_base();
        assert!(p.validar_usuario(41_192_387).is_ok());

        assert!(p.retirar_dinero(41_192_387, -5.0, MedioPago::Transferencia).is_err_and(|e| e == ErrorTransaccion::MontoInvalido));
        assert!(p.retirar_dinero(41_192_387, 0.0, MedioPago::Transferencia).is_err_and(|e| e == ErrorTransaccion::MontoInvalido));
        assert!(p.retirar_dinero(41_192_387, f64::MIN, MedioPago::Transferencia).is_err_and(|e| e == ErrorTransaccion::MontoInvalido));
        assert!(p.retirar_dinero(41_192_387, f64::INFINITY, MedioPago::Transferencia).is_err_and(|e| e == ErrorTransaccion::MontoInvalido));
        assert!(p.retirar_dinero(41_192_387, f64::NEG_INFINITY, MedioPago::Transferencia).is_err_and(|e| e == ErrorTransaccion::MontoInvalido));
        assert!(p.retirar_dinero(41_192_387, f64::NAN, MedioPago::Transferencia).is_err_and(|e| e == ErrorTransaccion::MontoInvalido));
        assert!(p.retirar_dinero(41_192_387, 0.0_f64.next_down(), MedioPago::Transferencia).is_err_and(|e| e == ErrorTransaccion::MontoInvalido));
    
        // comprobar que no se haya realizado ninguna transaccion
        assert!(p.get_balance_usuario(41_192_387).is_ok_and(|b| b == 0.0));
    }

    #[test]
    fn retirar_dinero_error_usuarios() {
        let mut p = crear_plataforma_base();
        assert!(p.retirar_dinero(41_192_387, 150.0, MedioPago::Transferencia).is_err_and(|e| e == ErrorTransaccion::UsuarioNoValidado));
        assert!(p.retirar_dinero(12_345_678, 999.0, MedioPago::Transferencia).is_err_and(|e| e == ErrorTransaccion::UsuarioInexistente));
    }

    #[test]
    fn retirar_dinero_balance_insuficiente() {
        let mut p = crear_plataforma_base();
        assert!(p.validar_usuario(41_192_387).is_ok());

        assert!(p.retirar_dinero(41_192_387, 300.0, MedioPago::Transferencia).is_err_and(|e| e == ErrorTransaccion::BalanceInsuficiente));
        assert!(p.retirar_dinero(41_192_387, f64::MAX, MedioPago::Transferencia).is_err_and(|e| e == ErrorTransaccion::BalanceInsuficiente));
        assert!(p.retirar_dinero(41_192_387, 0.0_f64.next_up(), MedioPago::Transferencia).is_err_and(|e| e == ErrorTransaccion::BalanceInsuficiente));

        assert!(p.get_balance_usuario(41_192_387).is_ok_and(|b| b == 0.0));
    }

    #[test]
    fn comprar_cripto_normal() {
        let mut p = crear_plataforma_base();
        assert!(p.validar_usuario(41_192_387).is_ok());

        assert!(p.ingresar_dinero(41_192_387, 1500.0).is_ok());

        let btc_coti = PlataformaXYZ::get_cotizacion(&Criptomoneda::Bitcoin);
        assert!(p.comprar_cripto(41_192_387, 0.002, Criptomoneda::Bitcoin).is_ok_and(
            |t| t.validar_compra_cripto(41_192_387, 0.002, Criptomoneda::Bitcoin, btc_coti) && 
                !t.validar_venta_cripto(41_192_387, 999.0, Criptomoneda::Bitcoin, 0.0)
        ));
        assert!(p.get_balance_usuario(41_192_387).is_ok_and(|b| b == 1500.0 - 0.002*btc_coti));
        assert!(p.get_balance_cripto_usuario(41_192_387, Criptomoneda::Bitcoin).is_ok_and(|b| b == 0.002));

        assert!(p.comprar_cripto(41_192_387, 0.0_f64.next_up(), Criptomoneda::Bitcoin).is_ok_and(
            |t| t.validar_compra_cripto(41_192_387, 0.0_f64.next_up(), Criptomoneda::Bitcoin, btc_coti)
        ));
        assert!(p.get_balance_usuario(41_192_387).is_ok_and(|b| b == 1500.0 - 0.002*btc_coti - 0.0_f64.next_up()*btc_coti));
        assert!(p.get_balance_cripto_usuario(41_192_387, Criptomoneda::Bitcoin).is_ok_and(|b| b == 0.002+0.0_f64.next_up()));
    }

    #[test]
    fn comprar_cripto_monto_invalido() {
        let mut p = crear_plataforma_base();
        assert!(p.validar_usuario(41_192_387).is_ok());

        assert!(p.comprar_cripto(41_192_387, -5.0, Criptomoneda::Bitcoin).is_err_and(|e| e == ErrorTransaccion::MontoInvalido));
        assert!(p.comprar_cripto(41_192_387, 0.0, Criptomoneda::Bitcoin).is_err_and(|e| e == ErrorTransaccion::MontoInvalido));
        assert!(p.comprar_cripto(41_192_387, f64::MIN, Criptomoneda::Bitcoin).is_err_and(|e| e == ErrorTransaccion::MontoInvalido));
        assert!(p.comprar_cripto(41_192_387, f64::INFINITY, Criptomoneda::Bitcoin).is_err_and(|e| e == ErrorTransaccion::MontoInvalido));
        assert!(p.comprar_cripto(41_192_387, f64::NEG_INFINITY, Criptomoneda::Bitcoin).is_err_and(|e| e == ErrorTransaccion::MontoInvalido));
        assert!(p.comprar_cripto(41_192_387, f64::NAN, Criptomoneda::Bitcoin).is_err_and(|e| e == ErrorTransaccion::MontoInvalido));
        assert!(p.comprar_cripto(41_192_387, 0.0_f64.next_down(), Criptomoneda::Bitcoin).is_err_and(|e| e == ErrorTransaccion::MontoInvalido));
    
        // comprobar que no se haya realizado ninguna transaccion
        assert!(p.get_balance_usuario(41_192_387).is_ok_and(|b| b == 0.0));
        assert!(p.get_balance_cripto_usuario(41_192_387, Criptomoneda::Bitcoin).is_ok_and(|b| b == 0.0));
    }

    #[test]
    fn comprar_cripto_error_usuarios() {
        let mut p = crear_plataforma_base();
        assert!(p.comprar_cripto(41_192_387, 0.015, Criptomoneda::Bitcoin).is_err_and(|e| e == ErrorTransaccion::UsuarioNoValidado));
        assert!(p.comprar_cripto(12_345_678, 0.015, Criptomoneda::Bitcoin).is_err_and(|e| e == ErrorTransaccion::UsuarioInexistente));
    }

    #[test]
    fn comprar_cripto_balance_insuficiente() {
        let mut p = crear_plataforma_base();
        assert!(p.validar_usuario(41_192_387).is_ok());

        assert!(p.comprar_cripto(41_192_387, 300.0, Criptomoneda::Bitcoin).is_err_and(|e| e == ErrorTransaccion::BalanceInsuficiente));
        assert!(p.comprar_cripto(41_192_387, f64::MAX, Criptomoneda::Bitcoin).is_err_and(|e| e == ErrorTransaccion::BalanceInsuficiente));
        assert!(p.comprar_cripto(41_192_387, 0.0_f64.next_up(), Criptomoneda::Bitcoin).is_err_and(|e| e == ErrorTransaccion::BalanceInsuficiente));
    
        // comprobar que no se haya realizado ninguna transaccion
        assert!(p.get_balance_usuario(41_192_387).is_ok_and(|b| b == 0.0));
        assert!(p.get_balance_cripto_usuario(41_192_387, Criptomoneda::Bitcoin).is_ok_and(|b| b == 0.0));
    }

    #[test]
    fn vender_cripto_normal() {
        let mut p = crear_plataforma_base();
        assert!(p.validar_usuario(41_192_387).is_ok());

        assert!(p.ingresar_dinero(41_192_387, 1000.0).is_ok());
        assert!(p.comprar_cripto(41_192_387, 200000.0, Criptomoneda::ShibaInu).is_ok());

        let shib_coti = PlataformaXYZ::get_cotizacion(&Criptomoneda::ShibaInu);
        assert!(p.vender_cripto(41_192_387, 100000.0, Criptomoneda::ShibaInu).is_ok_and(
            |t| t.validar_venta_cripto(41_192_387, 100000.0, Criptomoneda::ShibaInu, shib_coti) && 
                !t.validar_retiro_dinero(41_192_387, 999.0, MedioPago::MercadoPago)
        ));
        assert!(p.get_balance_usuario(41_192_387).is_ok_and(|b| b == 1000.0 - 200000.0*shib_coti + 100000.0*shib_coti));
        assert!(p.get_balance_cripto_usuario(41_192_387, Criptomoneda::ShibaInu).is_ok_and(|b| b == 200000.0 - 100000.0));

       assert!(p.vender_cripto(41_192_387, 0.0_f64.next_up(), Criptomoneda::ShibaInu).is_ok_and(
            |t| t.validar_venta_cripto(41_192_387, 0.0_f64.next_up(), Criptomoneda::ShibaInu, shib_coti)
        ));
        assert!(p.get_balance_usuario(41_192_387).is_ok_and(|b| b == 1000.0 - 200000.0*shib_coti + 100000.0*shib_coti + 0.0_f64.next_up()*shib_coti));
        assert!(p.get_balance_cripto_usuario(41_192_387, Criptomoneda::ShibaInu).is_ok_and(|b| b == 200000.0 - 100000.0 - 0.0_f64.next_up()));
    }

    #[test]
    fn vender_cripto_monto_invalido() {
        let mut p = crear_plataforma_base();
        assert!(p.validar_usuario(41_192_387).is_ok());

        assert!(p.vender_cripto(41_192_387, -5.0, Criptomoneda::Bitcoin).is_err_and(|e| e == ErrorTransaccion::MontoInvalido));
        assert!(p.vender_cripto(41_192_387, 0.0, Criptomoneda::Bitcoin).is_err_and(|e| e == ErrorTransaccion::MontoInvalido));
        assert!(p.vender_cripto(41_192_387, f64::MIN, Criptomoneda::Bitcoin).is_err_and(|e| e == ErrorTransaccion::MontoInvalido));
        assert!(p.vender_cripto(41_192_387, f64::INFINITY, Criptomoneda::Bitcoin).is_err_and(|e| e == ErrorTransaccion::MontoInvalido));
        assert!(p.vender_cripto(41_192_387, f64::NEG_INFINITY, Criptomoneda::Bitcoin).is_err_and(|e| e == ErrorTransaccion::MontoInvalido));
        assert!(p.vender_cripto(41_192_387, f64::NAN, Criptomoneda::Bitcoin).is_err_and(|e| e == ErrorTransaccion::MontoInvalido));
        assert!(p.vender_cripto(41_192_387, 0.0_f64.next_down(), Criptomoneda::Bitcoin).is_err_and(|e| e == ErrorTransaccion::MontoInvalido));
    
        // comprobar que no se haya realizado ninguna transaccion
        assert!(p.get_balance_usuario(41_192_387).is_ok_and(|b| b == 0.0));
        assert!(p.get_balance_cripto_usuario(41_192_387, Criptomoneda::Bitcoin).is_ok_and(|b| b == 0.0));
    }

    #[test]
    fn vender_cripto_error_usuarios() {
        let mut p = crear_plataforma_base();
        assert!(p.vender_cripto(41_192_387, 0.015, Criptomoneda::Bitcoin).is_err_and(|e| e == ErrorTransaccion::UsuarioNoValidado));
        assert!(p.vender_cripto(12_345_678, 0.015, Criptomoneda::Bitcoin).is_err_and(|e| e == ErrorTransaccion::UsuarioInexistente));
    }

    #[test]
    fn vender_cripto_balance_insuficiente() {
        let mut p = crear_plataforma_base();
        assert!(p.validar_usuario(41_192_387).is_ok());

        assert!(p.vender_cripto(41_192_387, 1.0, Criptomoneda::USDCoin).is_err_and(|e| e == ErrorTransaccion::BalanceInsuficiente));
        assert!(p.vender_cripto(41_192_387, f64::MAX, Criptomoneda::USDCoin).is_err_and(|e| e == ErrorTransaccion::BalanceInsuficiente));
        assert!(p.vender_cripto(41_192_387, 0.0_f64.next_up(), Criptomoneda::USDCoin).is_err_and(|e| e == ErrorTransaccion::BalanceInsuficiente));
    
        // comprobar que no se haya realizado ninguna transaccion
        assert!(p.get_balance_usuario(41_192_387).is_ok_and(|b| b == 0.0));
        assert!(p.get_balance_cripto_usuario(41_192_387, Criptomoneda::USDCoin).is_ok_and(|b| b == 0.0));
    }

    #[test]
    fn retirar_cripto_normal() {
        let mut p = crear_plataforma_base();
        assert!(p.validar_usuario(41_192_387).is_ok());

        assert!(p.ingresar_dinero(41_192_387, 1000000.0).is_ok());
        assert!(p.comprar_cripto(41_192_387, 500.0, Criptomoneda::Ethereum).is_ok());

        let eth_coti = PlataformaXYZ::get_cotizacion(&Criptomoneda::Ethereum);
        let dcm = DatosCriptomoneda::ethereum();
        let bc = dcm.blockchains.last().expect("Cripto no tiene blockchains");
        assert!(p.retirar_cripto(41_192_387, 250.0, Criptomoneda::Ethereum, bc).is_ok_and(
            |t| t.validar_retiro_cripto(41_192_387, 250.0, Criptomoneda::Ethereum, eth_coti, &bc.prefijo, "Ethereum304805589531039275") && 
                !t.validar_recepcion_cripto(41_192_387, 250.0, Criptomoneda::Ethereum, eth_coti, &bc.prefijo)
        ));
        assert!(p.get_balance_cripto_usuario(41_192_387, Criptomoneda::Ethereum).is_ok_and(|v| v == 500.0 - 250.0));
        
        assert!(p.retirar_cripto(41_192_387, 0.0_f64.next_up(), Criptomoneda::Ethereum, bc).is_ok_and(
            |t| t.validar_retiro_cripto(41_192_387, 0.0_f64.next_up(), Criptomoneda::Ethereum, eth_coti, &bc.prefijo, "Ethereum2147874104627985986")
        ));
        assert!(p.get_balance_cripto_usuario(41_192_387, Criptomoneda::Ethereum).is_ok_and(|v| v == 500.0 - 250.0 - 0.0_f64.next_up()));
    }

    #[test]
    fn retirar_cripto_monto_invalido() {
        let mut p = crear_plataforma_base();
        assert!(p.validar_usuario(41_192_387).is_ok());

        let dcm = DatosCriptomoneda::tether();
        let bc = dcm.blockchains.last().expect("Cripto no tiene blockchains");
        assert!(p.retirar_cripto(41_192_387, -5.0, Criptomoneda::Tether, bc).is_err_and(|e| e == ErrorTransaccion::MontoInvalido));
        assert!(p.retirar_cripto(41_192_387, 0.0, Criptomoneda::Tether, bc).is_err_and(|e| e == ErrorTransaccion::MontoInvalido));
        assert!(p.retirar_cripto(41_192_387, f64::MIN, Criptomoneda::Tether, bc).is_err_and(|e| e == ErrorTransaccion::MontoInvalido));
        assert!(p.retirar_cripto(41_192_387, f64::INFINITY, Criptomoneda::Tether, bc).is_err_and(|e| e == ErrorTransaccion::MontoInvalido));
        assert!(p.retirar_cripto(41_192_387, f64::NEG_INFINITY, Criptomoneda::Tether, bc).is_err_and(|e| e == ErrorTransaccion::MontoInvalido));
        assert!(p.retirar_cripto(41_192_387, f64::NAN, Criptomoneda::Tether, bc).is_err_and(|e| e == ErrorTransaccion::MontoInvalido));
        assert!(p.retirar_cripto(41_192_387, 0.0_f64.next_down(), Criptomoneda::Tether, bc).is_err_and(|e| e == ErrorTransaccion::MontoInvalido));
    
        // comprobar que no se haya realizado ninguna transaccion
        assert!(p.get_balance_usuario(41_192_387).is_ok_and(|b| b == 0.0));
        assert!(p.get_balance_cripto_usuario(41_192_387, Criptomoneda::Tether).is_ok_and(|b| b == 0.0));
    }

    #[test]
    fn retirar_cripto_error_usuarios() {
        let mut p = crear_plataforma_base();
        let dcm = DatosCriptomoneda::ethereum();
        let bc = dcm.blockchains.first().expect("Cripto no tiene blockchains");
        assert!(p.retirar_cripto(41_192_387, 0.015, Criptomoneda::Ethereum, bc).is_err_and(|e| e == ErrorTransaccion::UsuarioNoValidado));
        assert!(p.retirar_cripto(12_345_678, 0.015, Criptomoneda::Ethereum, bc).is_err_and(|e| e == ErrorTransaccion::UsuarioInexistente));
    }

    #[test]
    fn retirar_cripto_balance_insuficiente() {
        let mut p = crear_plataforma_base();
        assert!(p.validar_usuario(41_192_387).is_ok());

        let dcm = DatosCriptomoneda::usdcoin();
        let bc = dcm.blockchains.first().expect("Cripto no tiene blockchains");
        assert!(p.retirar_cripto(41_192_387, 1.0, Criptomoneda::USDCoin, bc).is_err_and(|e| e == ErrorTransaccion::BalanceInsuficiente));
        assert!(p.retirar_cripto(41_192_387, f64::MAX, Criptomoneda::USDCoin, bc).is_err_and(|e| e == ErrorTransaccion::BalanceInsuficiente));
        assert!(p.retirar_cripto(41_192_387, 0.0_f64.next_up(), Criptomoneda::USDCoin, bc).is_err_and(|e| e == ErrorTransaccion::BalanceInsuficiente));
    
        // comprobar que no se haya realizado ninguna transaccion
        assert!(p.get_balance_usuario(41_192_387).is_ok_and(|b| b == 0.0));
        assert!(p.get_balance_cripto_usuario(41_192_387, Criptomoneda::USDCoin).is_ok_and(|b| b == 0.0));
    }

    #[test]
    fn retirar_cripto_blockchain_invalida() {
        let mut p = crear_plataforma_base();
        assert!(p.validar_usuario(41_192_387).is_ok());

        assert!(p.retirar_cripto(41_192_387, 0.015, Criptomoneda::Bitcoin, &Blockchain::ethereum()).is_err_and(|e| e == ErrorTransaccion::BlockchainInvalida));
        assert!(p.retirar_cripto(41_192_387, 1.5, Criptomoneda::Ethereum, &Blockchain::polygon()).is_err_and(|e| e == ErrorTransaccion::BlockchainInvalida));
    }

    #[test]
    fn recibir_cripto() {
        let mut p = crear_plataforma_base();
        assert!(p.validar_usuario(41_192_387).is_ok());

        assert!(p.ingresar_dinero(41_192_387, 1000.0).is_ok());
        assert!(p.comprar_cripto(41_192_387, 500.0, Criptomoneda::USDCoin).is_ok());

        let usdc_coti: f64 = PlataformaXYZ::get_cotizacion(&Criptomoneda::USDCoin);
        let dcm = DatosCriptomoneda::usdcoin();
        let bc = dcm.blockchains.last().expect("Cripto no tiene blockchains");
        assert!(p.recibir_cripto(41_192_387, 250.0, Criptomoneda::USDCoin, bc).is_ok_and(
            |t| t.validar_recepcion_cripto(41_192_387, 250.0, Criptomoneda::USDCoin, usdc_coti, &bc.prefijo) && 
                !t.validar_retiro_cripto(41_192_387, 999.0, Criptomoneda::USDCoin, usdc_coti, &bc.prefijo, "Ethereum304805589531039275")
        ));
        assert!(p.get_balance_cripto_usuario(41_192_387, Criptomoneda::USDCoin).is_ok_and(|v| v == 500.0 + 250.0));

        assert!(p.recibir_cripto(41_192_387, 0.0_f64.next_up(), Criptomoneda::USDCoin, bc).is_ok_and(
            |t| t.validar_recepcion_cripto(41_192_387, 0.0_f64.next_up(), Criptomoneda::USDCoin, usdc_coti, &bc.prefijo)
        ));
        assert!(p.get_balance_cripto_usuario(41_192_387, Criptomoneda::USDCoin).is_ok_and(|v| v == 500.0 + 250.0 + 0.0_f64.next_up()));
    }

    #[test]
    fn recibir_cripto_monto_invalido() {
        let mut p = crear_plataforma_base();
        assert!(p.validar_usuario(41_192_387).is_ok());

        let dcm = DatosCriptomoneda::shibainu();
        let bc = dcm.blockchains.last().expect("Cripto no tiene blockchains");
        assert!(p.recibir_cripto(41_192_387, -5.0, Criptomoneda::ShibaInu, bc).is_err_and(|e| e == ErrorTransaccion::MontoInvalido));
        assert!(p.recibir_cripto(41_192_387, 0.0, Criptomoneda::ShibaInu, bc).is_err_and(|e| e == ErrorTransaccion::MontoInvalido));
        assert!(p.recibir_cripto(41_192_387, f64::MIN, Criptomoneda::ShibaInu, bc).is_err_and(|e| e == ErrorTransaccion::MontoInvalido));
        assert!(p.recibir_cripto(41_192_387, f64::INFINITY, Criptomoneda::ShibaInu, bc).is_err_and(|e| e == ErrorTransaccion::MontoInvalido));
        assert!(p.recibir_cripto(41_192_387, f64::NEG_INFINITY, Criptomoneda::ShibaInu, bc).is_err_and(|e| e == ErrorTransaccion::MontoInvalido));
        assert!(p.recibir_cripto(41_192_387, f64::NAN, Criptomoneda::ShibaInu, bc).is_err_and(|e| e == ErrorTransaccion::MontoInvalido));
        assert!(p.recibir_cripto(41_192_387, 0.0_f64.next_down(), Criptomoneda::ShibaInu, bc).is_err_and(|e| e == ErrorTransaccion::MontoInvalido));
    
        // comprobar que no se haya realizado ninguna transaccion
        assert!(p.get_balance_usuario(41_192_387).is_ok_and(|b| b == 0.0));
        assert!(p.get_balance_cripto_usuario(41_192_387, Criptomoneda::ShibaInu).is_ok_and(|b| b == 0.0));
    }

    #[test]
    fn recibir_cripto_error_usuarios() {
        let mut p = crear_plataforma_base();
        let dcm = DatosCriptomoneda::bitcoin();
        let bc = dcm.blockchains.first().expect("Cripto no tiene blockchains");
        assert!(p.recibir_cripto(41_192_387, 0.015, Criptomoneda::Bitcoin, bc).is_err_and(|e| e == ErrorTransaccion::UsuarioNoValidado));
        assert!(p.recibir_cripto(12_345_678, 0.015, Criptomoneda::Bitcoin, bc).is_err_and(|e| e == ErrorTransaccion::UsuarioInexistente));
    }

    #[test]
    fn recibir_cripto_blockchain_invalida() {
        let mut p = crear_plataforma_base();
        assert!(p.validar_usuario(41_192_387).is_ok());

        assert!(p.recibir_cripto(41_192_387, 0.015, Criptomoneda::USDCoin, &Blockchain::tron()).is_err_and(|e| e == ErrorTransaccion::BlockchainInvalida));
        assert!(p.recibir_cripto(41_192_387, 1.5, Criptomoneda::ShibaInu, &Blockchain::solana()).is_err_and(|e| e == ErrorTransaccion::BlockchainInvalida));
    }

    #[test]
    fn cripto_mas_vendida() {
        let mut p = crear_plataforma_base_compras_ventas();
        assert_eq!(p.cripto_mas_vendida(), DatosCriptomoneda::shibainu());

        assert!(p.comprar_cripto(41_192_387, 0.1, Criptomoneda::Tether).is_ok());
        for _ in 0..4 {
            assert!(p.vender_cripto(41_192_387, 0.001, Criptomoneda::Tether).is_ok());
        }
        assert_eq!(p.cripto_mas_vendida(), DatosCriptomoneda::tether());

        assert!(p.comprar_cripto(41_192_387, 1.0, Criptomoneda::USDCoin).is_ok());
        for _ in 0..7 {
            assert!(p.vender_cripto(41_192_387, 0.001, Criptomoneda::USDCoin).is_ok());
        }
        assert_eq!(p.cripto_mas_vendida(), DatosCriptomoneda::usdcoin());
    }

    #[test]
    fn cripto_mas_comprada() {
        let mut p = crear_plataforma_base_compras_ventas();
        assert_eq!(p.cripto_mas_comprada(), DatosCriptomoneda::ethereum());

        for _ in 0..4 {
            assert!(p.comprar_cripto(41_192_387, 0.1, Criptomoneda::Tether).is_ok());
        }
        assert_eq!(p.cripto_mas_comprada(), DatosCriptomoneda::tether());

        for _ in 0..7 {
            assert!(p.comprar_cripto(41_192_387, 1.0, Criptomoneda::USDCoin).is_ok());
        }
        assert_eq!(p.cripto_mas_comprada(), DatosCriptomoneda::usdcoin());
    }

    #[test]
    fn cripto_mas_volumen_ventas() {
        let mut p = crear_plataforma_base_compras_ventas();
        assert_eq!(p.cripto_mas_volumen_ventas(), DatosCriptomoneda::ethereum());

        assert!(p.comprar_cripto(41_192_387, 1_000_000.0, Criptomoneda::Tether).is_ok());
        assert!(p.vender_cripto(41_192_387, 1_000_000.0, Criptomoneda::Tether).is_ok());
        assert_eq!(p.cripto_mas_volumen_ventas(), DatosCriptomoneda::tether());

        assert!(p.comprar_cripto(41_192_387, 2_000_000.0, Criptomoneda::USDCoin).is_ok());
        assert!(p.vender_cripto(41_192_387, 2_000_000.0, Criptomoneda::USDCoin).is_ok());
        assert_eq!(p.cripto_mas_volumen_ventas(), DatosCriptomoneda::usdcoin());
    }

    #[test]
    fn cripto_mas_volumen_compras() {
        let mut p = crear_plataforma_base_compras_ventas();
        assert_eq!(p.cripto_mas_volumen_compras(), DatosCriptomoneda::bitcoin());

        assert!(p.comprar_cripto(41_192_387, 1_000_000.0, Criptomoneda::Tether).is_ok());
        assert_eq!(p.cripto_mas_volumen_compras(), DatosCriptomoneda::tether());

        assert!(p.comprar_cripto(41_192_387, 2_000_000.0, Criptomoneda::USDCoin).is_ok());
        assert_eq!(p.cripto_mas_volumen_compras(), DatosCriptomoneda::usdcoin());
    }
}
