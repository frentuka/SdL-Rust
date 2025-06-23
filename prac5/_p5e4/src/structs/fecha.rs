use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::fmt;
use serde::{Deserialize, Serialize};

const NOMBRE_MESES: [&str; 12] = ["Enero", "Febrero", "Marzo", "Abril",
    "Mayo", "Junio", "Julio", "Agosto",
    "Septiembre", "Octubre", "Noviembre", "Diciembre"];
#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Debug)]
pub struct Fecha {
    pub(crate) dia: u8,
    pub(crate) mes: u8,
    pub(crate) ano: i64
}

impl Default for Fecha {
    fn default() -> Self {
        Fecha { dia: 1, mes: 1, ano: 0 }
    }
}

impl PartialOrd for Fecha {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.dia == other.dia
            && self.mes == other.mes
            && self.ano == other.ano
        { return Some(Equal) }

        if self.ano > other.ano { return Some(Greater) }
        if self.mes > other.mes { return Some(Greater) }
        if self.dia > other.dia { return Some(Greater) }

        Some(Less)
    }
}

impl fmt::Display for Fecha {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.es_fecha_valida() {
            write!(f, "{} de {} del {}", self.dia, NOMBRE_MESES[self.mes as usize - 1], self.ano)
        } else {
            write!(f, "{}/{}/{}", self.dia, self.mes, self.ano)
        }
    }
}

impl Fecha {

    // El año podría ser negativo, indicando días antes de Cristo.
    pub fn new(dia: u8, mes: u8, ano: i64) -> Option<Fecha> {
        let fecha = Fecha { dia, mes, ano };
        if fecha.es_fecha_valida() {
            return Some(fecha);
        }
        None
    }

    pub fn es_fecha_valida(&self) -> bool {
        // check que el mes sea válido
        if !(1..=12).contains(&self.mes) { return false }

        // check días del mes
        if self.dia == 0
            || self.dia > self.dias_mes_actual()
        { return false }

        // el año no puede ser incorrecto...
        // a no ser que se contabilice la edad del universo
        // que dudo mucho que pueda importar para este caso
        true
    }

    pub fn es_bisiesto(&self) -> bool {
        self.ano % 4 == 0
    }

    pub fn sumar_dias(&mut self, dias: u32) {
        let mut dias_restantes = dias;

        while dias_restantes > 0 {
            let dias_mes_actual = self.dias_mes_actual();
            let dias_para_proximo_mes = u32::from(dias_mes_actual - self.dia + 1);

            if dias_restantes >= dias_para_proximo_mes {
                // ir al siguiente mes

                dias_restantes-= dias_para_proximo_mes;
                self.dia = 1;
                self.mes += 1;

                if self.mes > 12 {
                    self.mes = 1;
                    self.ano+= 1;
                }
            } else {
                self.dia += u8::try_from(dias_restantes).unwrap_or(0);
                dias_restantes = 0;
            }
        }
    }

    pub fn restar_dias(&mut self, dias: u32) {
        let mut dias_restantes = dias;

        while dias_restantes > 0 {
            if dias_restantes >= u32::from(self.dia) {
                // ir al anterior mes
                dias_restantes-= u32::from(self.dia);
                self.mes-= 1;

                if self.mes < 1 {
                    self.mes = 12;
                    self.ano-= 1;
                }

                // corregir self.dia == 0
                self.dia = self.dias_mes_actual();
            } else {
                self.dia-= u8::try_from(dias_restantes).unwrap_or(0);
                dias_restantes = 0;
            }
        }
    }

    pub fn dias_mes_actual(&self) -> u8 {
        match self.mes {
            4 | 6 | 9 | 11 => 30,
            2 => if self.es_bisiesto() { 29 } else { 28 },
            _ => 31,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_default() {
        // default: 01/01/0000
        let default_fecha = Fecha::default();
        assert_eq!(default_fecha.dia, 1);
        assert_eq!(default_fecha.mes, 1);
        assert_eq!(default_fecha.ano, 0);
    }

    #[test]
    fn test_display() {
        let valid_fecha = Fecha::default();
        let invalid_fecha = Fecha { dia: 0, mes: 1, ano: 0 };

        // write!(f, "{} de {} del {}", self.dia, NOMBRE_MESES[self.mes as usize - 1], self.ano)

        assert_ne!(format!("{valid_fecha}"), format!("{}", invalid_fecha));
        assert_eq!(format!("{valid_fecha}"), format!("{} de {} del {}", valid_fecha.dia, NOMBRE_MESES[valid_fecha.mes as usize - 1], valid_fecha.ano));
        assert_eq!(format!("{invalid_fecha}"), format!("{}/{}/{}", invalid_fecha.dia, invalid_fecha.mes, invalid_fecha.ano));
    }

    #[test]
    fn test_new() {
        // invalida
        let fecha = Fecha::new(0, 0, 0);
        assert!(fecha.is_none());

        // valida
        let fecha = Fecha::new(22, 08, 2002);
        assert!(fecha.is_some());
    }

    #[test]
    fn test_bisiesto() {
        let Some(fecha) = Fecha::new(1, 1, 0) else { panic!() };
        assert!(fecha.es_bisiesto());

        let Some(fecha) = Fecha::new(1, 1, 2000) else { panic!() };
        assert!(fecha.es_bisiesto());

        let Some(fecha) = Fecha::new(1, 1, -4) else { panic!() };
        assert!(fecha.es_bisiesto());

        let Some(fecha) = Fecha::new(1, 1, 1) else { panic!() };
        assert!(!fecha.es_bisiesto());
    }

    #[test]
    fn test_restar_dias() {
        let Some(mut fecha) = Fecha::new(30, 04, 2016) else { panic!() };

        fecha.restar_dias(5000);

        assert_eq!(fecha.dia, 22);
        assert_eq!(fecha.mes, 08);
        assert_eq!(fecha.ano, 2002);
    }

    #[test]
    fn test_sumar_dias() {
        let Some(mut fecha) = Fecha::new(22, 08, 2002) else { panic!() };

        fecha.sumar_dias(5000);

        assert_eq!(fecha.dia, 30);
        assert_eq!(fecha.mes, 04);
        assert_eq!(fecha.ano, 2016);
    }

    #[test]
    fn test_dias_mes_actual() {
        let Some(fecha) = Fecha::new(22, 01, 2002) else { panic!() };
        assert_eq!(fecha.dias_mes_actual(), 31);
        let Some(fecha) = Fecha::new(22, 02, 2002) else { panic!() };
        assert_eq!(fecha.dias_mes_actual(), 28);
        let Some(fecha) = Fecha::new(22, 02, 2004) else { panic!() };
        assert_eq!(fecha.dias_mes_actual(), 29);
        let Some(fecha) = Fecha::new(22, 03, 2002) else { panic!() };
        assert_eq!(fecha.dias_mes_actual(), 31);
        let Some(fecha) = Fecha::new(22, 04, 2002) else { panic!() };
        assert_eq!(fecha.dias_mes_actual(), 30);
        let Some(fecha) = Fecha::new(22, 05, 2002) else { panic!() };
        assert_eq!(fecha.dias_mes_actual(), 31);
        let Some(fecha) = Fecha::new(22, 06, 2002) else { panic!() };
        assert_eq!(fecha.dias_mes_actual(), 30);
        let Some(fecha) = Fecha::new(22, 07, 2002) else { panic!() };
        assert_eq!(fecha.dias_mes_actual(), 31);
        let Some(fecha) = Fecha::new(22, 08, 2002) else { panic!() };
        assert_eq!(fecha.dias_mes_actual(), 31);
        let Some(fecha) = Fecha::new(22, 09, 2002) else { panic!() };
        assert_eq!(fecha.dias_mes_actual(), 30);
        let Some(fecha) = Fecha::new(22, 10, 2002) else { panic!() };
        assert_eq!(fecha.dias_mes_actual(), 31);
        let Some(fecha) = Fecha::new(22, 11, 2002) else { panic!() };
        assert_eq!(fecha.dias_mes_actual(), 30);
        let Some(fecha) = Fecha::new(22, 12, 2002) else { panic!() };
        assert_eq!(fecha.dias_mes_actual(), 31);
    }
}