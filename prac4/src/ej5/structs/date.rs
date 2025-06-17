use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::fmt;

const NOMBRE_MESES: [&str; 12] = ["Enero", "Febrero", "Marzo", "Abril",
    "Mayo", "Junio", "Julio", "Agosto",
    "Septiembre", "Octubre", "Noviembre", "Diciembre"];
#[derive(Clone, PartialEq, Debug, Copy, Hash)]
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
            let dias_para_proximo_mes = (dias_mes_actual - self.day + 1) as u32;

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
                self.day += dias_restantes as u8;
                dias_restantes = 0;
            }
        }
    }

    pub fn subtract_days(&mut self, dias: u32) {
        let mut dias_restantes = dias;

        while dias_restantes > 0 {
            if dias_restantes >= self.day as u32 {
                // ir al anterior mes
                dias_restantes-= self.day as u32;
                self.month -= 1;

                if self.month < 1 {
                    self.month = 12;
                    self.year -= 1;
                }

                // corregir self.dia == 0
                self.day = self.current_month_days();
            } else {
                self.day -= dias_restantes as u8;
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