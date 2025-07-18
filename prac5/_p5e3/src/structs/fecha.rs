use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::fmt;
use serde::{Deserialize, Serialize};

const NOMBRE_MESES: [&str; 12] = ["Enero", "Febrero", "Marzo", "Abril",
    "Mayo", "Junio", "Julio", "Agosto",
    "Septiembre", "Octubre", "Noviembre", "Diciembre"];
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
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
            let dias_para_proximo_mes = (dias_mes_actual - self.dia + 1) as u32;

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
                self.dia+= dias_restantes as u8;
                dias_restantes = 0;
            }
        }
    }

    pub fn restar_dias(&mut self, dias: u32) {
        let mut dias_restantes = dias;

        while dias_restantes > 0 {
            if dias_restantes >= self.dia as u32 {
                // ir al anterior mes
                dias_restantes-= self.dia as u32;
                self.mes-= 1;

                if self.mes < 1 {
                    self.mes = 12;
                    self.ano-= 1;
                }

                // corregir self.dia == 0
                self.dia = self.dias_mes_actual();
            } else {
                self.dia-= dias_restantes as u8;
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