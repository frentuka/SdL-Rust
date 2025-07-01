use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::fmt;
use serde::{Deserialize, Serialize};

const NOMBRE_MESES: [&str; 12] = ["Enero", "Febrero", "Marzo", "Abril",
    "Mayo", "Junio", "Julio", "Agosto",
    "Septiembre", "Octubre", "Noviembre", "Diciembre"];
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, Copy, Hash)]
pub struct Date {
    pub(crate) day: u8,
    pub(crate) month: u8,
    pub(crate) year: i64
}

impl Default for Date {
    fn default() -> Self {
        Date { day: 1, month: 1, year: 0 }
    }
}

impl PartialOrd for Date {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.day == other.day
            && self.month == other.month
            && self.year == other.year
        { return Some(Equal) }

        if self.year > other.year { return Some(Greater) }
        if self.month > other.month { return Some(Greater) }
        if self.day > other.day { return Some(Greater) }

        Some(Less)
    }
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_date_valid() {
            write!(f, "{} de {} del {}", self.day, NOMBRE_MESES[self.month as usize - 1], self.year)
        } else {
            write!(f, "{}/{}/{}", self.day, self.month, self.year)
        }
    }
}

impl Date {
    // El año podría ser negativo, indicando época antes de Cristo.
    pub fn new(dia: u8, mes: u8, ano: i64) -> Option<Date> {
        let fecha = Date { day: dia, month: mes, year: ano };
        if fecha.is_date_valid() {
            return Some(fecha);
        }
        None
    }

    pub fn is_date_valid(&self) -> bool {
        // check que el mes sea válido
        if !(1..=12).contains(&self.month) { return false }

        // check días del mes
        if self.day == 0
            || self.day > self.current_month_days()
        { return false }

        // el año no puede ser incorrecto...
        // a no ser que se contabilice la edad del universo
        // que dudo mucho que pueda importar para este caso
        true
    }

    pub fn is_leap_year(&self) -> bool {
        self.year % 4 == 0
    }

    pub fn add_days(&mut self, dias: u32) {
        let mut dias_restantes = dias;

        while dias_restantes > 0 {
            let dias_mes_actual = self.current_month_days();
            let dias_para_proximo_mes = u32::from(dias_mes_actual - self.day + 1);

            if dias_restantes >= dias_para_proximo_mes {
                // ir al siguiente mes

                dias_restantes-= dias_para_proximo_mes;
                self.day = 1;
                self.month += 1;

                if self.month > 12 {
                    self.month = 1;
                    self.year += 1;
                }
            } else {
                self.day += u8::try_from(dias_restantes).unwrap();
                dias_restantes = 0;
            }
        }
    }

    pub fn subtract_days(&mut self, dias: u32) {
        let mut dias_restantes = dias;

        while dias_restantes > 0 {
            if dias_restantes >= u32::from(self.day) {
                // ir al anterior mes
                dias_restantes-= u32::from(self.day);
                self.month -= 1;

                if self.month < 1 {
                    self.month = 12;
                    self.year -= 1;
                }

                // corregir self.dia == 0
                self.day = self.current_month_days();
            } else {
                self.day -= u8::try_from(dias_restantes).unwrap();
                dias_restantes = 0;
            }
        }
    }

    pub fn current_month_days(&self) -> u8 {
        match self.month {
            4 | 6 | 9 | 11 => 30,
            2 => if self.is_leap_year() { 29 } else { 28 },
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
        let default_fecha = Date::default();
        assert_eq!(default_fecha.day, 1);
        assert_eq!(default_fecha.month, 1);
        assert_eq!(default_fecha.year, 0);
    }

    #[test]
    fn test_display() {
        let valid_fecha = Date::default();
        let invalid_fecha = Date { day: 0, month: 1, year: 0 };

        // write!(f, "{} de {} del {}", self.dia, NOMBRE_MESES[self.mes as usize - 1], self.ano)

        assert_ne!(format!("{valid_fecha}"), format!("{}", invalid_fecha));
        assert_eq!(format!("{valid_fecha}"), format!("{} de {} del {}", valid_fecha.day, NOMBRE_MESES[valid_fecha.month as usize - 1], valid_fecha.year));
        assert_eq!(format!("{invalid_fecha}"), format!("{}/{}/{}", invalid_fecha.day, invalid_fecha.month, invalid_fecha.year));
    }

    #[test]
    fn test_new() {
        // invalida
        let fecha = Date::new(0, 0, 0);
        assert!(fecha.is_none());

        // valida
        let fecha = Date::new(22, 08, 2002);
        assert!(fecha.is_some());
    }

    #[test]
    fn test_bisiesto() {
        let Some(fecha) = Date::new(1, 1, 0) else { panic!() };
        assert!(fecha.is_leap_year());

        let Some(fecha) = Date::new(1, 1, 2000) else { panic!() };
        assert!(fecha.is_leap_year());

        let Some(fecha) = Date::new(1, 1, -4) else { panic!() };
        assert!(fecha.is_leap_year());

        let Some(fecha) = Date::new(1, 1, 1) else { panic!() };
        assert!(!fecha.is_leap_year());
    }

    #[test]
    fn test_restar_dias() {
        let Some(mut fecha) = Date::new(30, 04, 2016) else { panic!() };

        fecha.subtract_days(5000);

        assert_eq!(fecha.day, 22);
        assert_eq!(fecha.month, 08);
        assert_eq!(fecha.year, 2002);
    }

    #[test]
    fn test_sumar_dias() {
        let Some(mut fecha) = Date::new(22, 08, 2002) else { panic!() };

        fecha.add_days(5000);

        assert_eq!(fecha.day, 30);
        assert_eq!(fecha.month, 04);
        assert_eq!(fecha.year, 2016);
    }

    #[test]
    fn test_current_month_days() {
        let Some(fecha) = Date::new(22, 01, 2002) else { panic!() };
        assert_eq!(fecha.current_month_days(), 31);
        let Some(fecha) = Date::new(22, 02, 2002) else { panic!() };
        assert_eq!(fecha.current_month_days(), 28);
        let Some(fecha) = Date::new(22, 02, 2004) else { panic!() };
        assert_eq!(fecha.current_month_days(), 29);
        let Some(fecha) = Date::new(22, 03, 2002) else { panic!() };
        assert_eq!(fecha.current_month_days(), 31);
        let Some(fecha) = Date::new(22, 04, 2002) else { panic!() };
        assert_eq!(fecha.current_month_days(), 30);
        let Some(fecha) = Date::new(22, 05, 2002) else { panic!() };
        assert_eq!(fecha.current_month_days(), 31);
        let Some(fecha) = Date::new(22, 06, 2002) else { panic!() };
        assert_eq!(fecha.current_month_days(), 30);
        let Some(fecha) = Date::new(22, 07, 2002) else { panic!() };
        assert_eq!(fecha.current_month_days(), 31);
        let Some(fecha) = Date::new(22, 08, 2002) else { panic!() };
        assert_eq!(fecha.current_month_days(), 31);
        let Some(fecha) = Date::new(22, 09, 2002) else { panic!() };
        assert_eq!(fecha.current_month_days(), 30);
        let Some(fecha) = Date::new(22, 10, 2002) else { panic!() };
        assert_eq!(fecha.current_month_days(), 31);
        let Some(fecha) = Date::new(22, 11, 2002) else { panic!() };
        assert_eq!(fecha.current_month_days(), 30);
        let Some(fecha) = Date::new(22, 12, 2002) else { panic!() };
        assert_eq!(fecha.current_month_days(), 31);
    }

    #[test]
    fn test_cmp() {
        let fecha1 = Date { day: 1, month: 1, year: 1};
        let fecha2 = Date { day: 3, month: 1, year: 1};
        let fecha3 = Date { day: 3, month: 1, year: 1};

        assert!(fecha1 < fecha2, "Fecha 1 es anterior, por ende, es menor");
        assert_eq!(fecha3, fecha2, "Fecha 3 es igual a fecha 2");
        assert!(fecha3 > fecha1, "Fecha 3 es posterior a fecha1, por ende, es mayor");
    }
}