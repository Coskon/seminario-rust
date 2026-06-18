#![allow(unused)]
#[derive(Debug, Clone)] // TODO
pub struct Fecha {
    pub dia: u32,
    pub mes: u32,
    pub anio: u64
}

impl Fecha {
    pub fn new(dia: u32, mes: u32, anio: u64) -> Self {
        Fecha { dia, mes, anio }
    }

    fn dias_del_mes(&self) -> [u32; 12] {
        let febrero: u32 = if self.es_bisiesto() { 29 } else { 28 };
        [31, febrero, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    }

    pub fn es_bisiesto(&self) -> bool {
        self.anio % 4 == 0 && (self.anio % 100 != 0 || self.anio % 400 == 0)
    }

    pub fn es_fecha_valida(&self) -> bool {
        let dias_por_mes = self.dias_del_mes();
        self.mes > 0 && self.mes <= 12 && self.dia > 0 && self.dia <= dias_por_mes[(self.mes-1) as usize]
    }

    pub fn sumar_dias(&mut self, mut dias: u32) {
        if !self.es_fecha_valida() {
            panic!("Fecha invalida")
        }

        while dias > 0 {
            // calculo en cada iteracion por si el año cambia a uno bisiesto
            let dias_por_mes = self.dias_del_mes();
            let dias_mes_actual: u32 = dias_por_mes[(self.mes-1) as usize];

            let restantes = dias_mes_actual - self.dia;
            if dias <= restantes {
                self.dia += dias;
                break;
            } else {
                dias -= restantes + 1;
                self.dia = 1;
                self.mes += 1;
                if self.mes > 12 {
                    self.mes = 1;
                    self.anio += 1;
                }
            }
        }
    }

    pub fn restar_dias(&mut self, mut dias: u32) {
        if !self.es_fecha_valida() {
            panic!("Fecha invalida")
        }

        while dias > 0 {
            if dias < self.dia {
                self.dia -= dias;
                break;
            } else {
                dias -= self.dia;

                self.mes -= 1;
                if self.mes == 0 {
                    self.mes = 12;
                    self.anio -= 1;
                }

                let dias_por_mes = self.dias_del_mes();
                let dias_mes_actual: u32 = dias_por_mes[(self.mes-1) as usize];
                
                self.dia = dias_mes_actual;
            }
        }
    }

    pub fn es_mayor(&self, other: &Fecha) -> bool {
        if !self.es_fecha_valida() || !other.es_fecha_valida() {
            panic!("Fechas invalidas a comparar")
        }
        self.anio > other.anio || 
        (self.anio == other.anio && self.mes > other.mes) ||
        (self.anio == other.anio && self.mes == other.mes && self.dia > other.dia)
    }

    pub fn equals_fecha(&self, dia: u32, mes: u32, anio: u64) -> bool {
        self.dia == dia && self.mes == mes && self.anio == anio
    }

    pub fn not_equals_fecha(&self, dia: u32, mes: u32, anio: u64) -> bool {
        !self.equals_fecha(dia, mes, anio)
    }

    pub fn equals(&self, other: &Self) -> bool {
        self.equals_fecha(other.dia, other.mes, other.anio)
    }
    pub fn not_equals(&self, other: &Self) -> bool {
        !self.equals(other)
    }

    // para usar en el ejercicio 10
    pub fn fecha_actual() -> Fecha {
        Fecha::new(17, 5, 2026)
    }
}

#[test]
fn test_fecha_equals() {
    let f = Fecha::new(2, 6, 2004);
    assert!(f.equals(&Fecha::new(2, 6, 2004)));
    assert!(f.equals_fecha(2, 6, 2004));
    assert!(f.not_equals(&Fecha::new(3, 6, 2004)));
    assert!(f.not_equals_fecha(3, 6, 2004));
    assert!(f.not_equals(&Fecha::new(2, 7, 2004)));
    assert!(f.not_equals_fecha(2, 7, 2004));
    assert!(f.not_equals(&Fecha::new(2, 6, 2005)));
    assert!(f.not_equals_fecha(2, 6, 2005));
}

#[test]
fn test_fecha_es_valida() {
    let f = Fecha::new(15, 12, 2026);
    assert!(f.es_fecha_valida());

    let f = Fecha::new(32, 12, 2026); // dia 32
    assert!(!f.es_fecha_valida());

    let f = Fecha::new(31, 2, 2026); // 31 de febrero
    assert!(!f.es_fecha_valida());

    let f = Fecha::new(15, 14, 2026); // mes 14
    assert!(!f.es_fecha_valida());

    let f = Fecha::new(29, 2, 2026); // 29 de febrero año no bisiesto
    assert!(!f.es_fecha_valida());

    let f = Fecha::new(29, 2, 2024); // 29 de febrero año bisiesto
    assert!(f.es_fecha_valida());
}

#[test]
fn test_fecha_bisiesto() {
    let f = Fecha::new(15, 12, 2026);
    assert!(!f.es_bisiesto());

    let f = Fecha::new(15, 12, 2024);
    assert!(f.es_bisiesto());

    let f = Fecha::new(15, 12, 2000);
    assert!(f.es_bisiesto());

    let f = Fecha::new(15, 12, 1600);
    assert!(f.es_bisiesto());

    let f = Fecha::new(15, 12, 1700);
    assert!(!f.es_bisiesto());
}

#[test]
fn test_fecha_sumar_dias() {
    let mut f = Fecha::new(15, 12, 2026);
    f.sumar_dias(4); // suma normal
    assert!(f.equals_fecha(19, 12, 2026));
    f.sumar_dias(12); // se queda al borde de cambiar de mes/año
    assert!(f.equals_fecha(31, 12, 2026));
    f.sumar_dias(1); // pasa de año
    assert!(f.equals_fecha(1, 1, 2027));
    f.sumar_dias(365); // todo un año
    assert!(f.equals_fecha(1, 1, 2028));
    f.sumar_dias(60); // como 2028 es bisiesto, queda 1/3/2028 en lugar de 2/3/2028
    assert!(f.equals_fecha(1, 3, 2028));

    let mut f = Fecha::new(1, 1, 2025);
    f.sumar_dias(60); // como 2025 no es bisiesto, queda 2/3/2025 en lugar de 1/3/2025
    assert!(f.equals_fecha(2, 3, 2025));
}

#[should_panic(expected="Fecha invalida")]
#[test]
fn test_fecha_sumar_dias_invalido() {
    let mut f = Fecha::new(100, 27, 2025);
    f.sumar_dias(10);
}

#[test]
fn test_fecha_restar_dias() {
    let mut f = Fecha::new(15, 12, 2026);
    f.restar_dias(6); // resta normal
    assert!(f.equals_fecha(9, 12, 2026));
    f.restar_dias(8); // se queda en el primer dia
    assert!(f.equals_fecha(1, 12, 2026));
    f.restar_dias(1); // cambia al siguiente mes con la cantidad de dias de ese mes
    assert!(f.equals_fecha(30, 11, 2026));
    f.restar_dias(365); // todo un año
    assert!(f.equals_fecha(30, 11, 2025));
    f.restar_dias(365*2u32); // dos años donde uno de los años es bisiesto
    assert!(f.equals_fecha(1, 12, 2023));
}

#[should_panic(expected="Fecha invalida")]
#[test]
fn test_fecha_restar_dias_invalido() {
    let mut f = Fecha::new(100, 27, 2025);
    f.restar_dias(10);
}

#[test]
fn test_fecha_es_mayor() {
    let f1 = Fecha::new(16, 12, 2026);
    let f2 = Fecha::new(15, 12, 2026);
    assert!(f1.es_mayor(&f2)); // mayor por 1 dia

    let f1 = Fecha::new(10, 12, 2026);
    let f2 = Fecha::new(15, 12, 2026);
    assert!(!f1.es_mayor(&f2)); // menor

    let f1 = Fecha::new(15, 12, 2026);
    let f2 = Fecha::new(15, 11, 2026);
    assert!(f1.es_mayor(&f2)); // mayor por 1 mes

    let f1 = Fecha::new(15, 12, 2027);
    let f2 = Fecha::new(15, 12, 2026);
    assert!(f1.es_mayor(&f2)); // mayor por 1 año

    let f1 = Fecha::new(15, 12, 2026);
    let f2 = Fecha::new(15, 12, 2026);
    assert!(!f1.es_mayor(&f2)); // iguales
}

#[should_panic(expected="Fechas invalidas a comparar")]
#[test]
fn test_fecha_es_mayor_self_invalido() {
    let f1 = Fecha::new(67, 21, 2026);
    let f2 = Fecha::new(15, 12, 2026);
    f1.es_mayor(&f2);
}

#[should_panic(expected="Fechas invalidas a comparar")]
#[test]
fn test_fecha_es_mayor_other_invalido() {
    let f1 = Fecha::new(15, 12, 2026);
    let f2 = Fecha::new(1, 69, 2026);
    f1.es_mayor(&f2);
}