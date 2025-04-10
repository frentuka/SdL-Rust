
/*
    3- Escribir un programa que defina una estructura Fecha que tenga campos para:
        - día
        - mes
        - año
        Para dicha estructura implemente los siguientes métodos:
        ➢ new: que pasando los parámetros correspondientes, crea una Fecha y la retorna.
        ➢ es_fecha_valida: retorna true si es una fecha valida, false caso contrario.//tenga en
        cuenta los años bisiestos también.
        ➢ es_bisiesto: retorna true si el año de la fecha pertenece a un año bisiesto.
        ➢ sumar_dias(dias): suma la cantidad de días a la fecha, modificándose
        ➢ restar_dias(dias): resta la cantidad de días a la fecha, modificándose
        ➢ es_mayor(una_fecha): que retorna true si la fecha que recibe el mensaje es mayor a
        la fecha pasada por parámetro..
 */
use std::fmt;

// podría usarse sólo DIAS_POR_MES pero prefiero gastar memoria y ahorrarme años de vida
const DIAS_POR_MES: [u8; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
const DIAS_POR_MES_BIS: [u8; 12] = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
const NOMBRE_MESES: [&str; 12] = ["Enero", "Febrero", "Marzo", "Abril",
                                  "Mayo", "Junio", "Julio", "Agosto",
                                  "Septiembre", "Octubre", "Noviembre", "Diciembre"];
#[derive(Debug)]
struct Fecha {
    dia: u8,
    mes: u8,
    ano: i64
}

impl fmt::Display for Fecha {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.es_fecha_valida() {
            write!(f, "{} de {} de {}", self.dia, NOMBRE_MESES[self.mes as usize - 1], self.ano)
        } else {
            write!(f, "{}/{}/{}", self.dia, self.mes, self.ano)
        }
    }
}

impl Fecha {

    // El año podría ser negativo, indicando días antes de Cristo.
    fn new(dia: u8, mes: u8, ano: i64) -> Fecha {
        Fecha { dia, mes, ano }
    }

    fn es_fecha_valida(&self) -> bool {
        // check que el mes sea válido
        if !(1..=12).contains(&self.mes) { return false }

        // check días del mes
        if self.es_bisiesto() && !(1..=DIAS_POR_MES_BIS[self.mes as usize -1]).contains(&self.mes)
            ||
          !self.es_bisiesto() && !(1..=DIAS_POR_MES[self.mes as usize -1]).contains(&self.mes)
        { return false }

        // el año no puede ser incorrecto...
        // a no ser que se contabilice la edad del universo
        // que dudo mucho que pueda importar para este caso
        true
    }

    fn es_bisiesto(&self) -> bool {
        self.ano % 4 == 0
    }

    // este código es un fracaso porque no tiene en cuenta si se atravesará o no un 29/feb
    // en cambio, erroneamente, calcula si el año actual es bisiesto
    // el problema: si el año actual es bisiesto pero el 29/feb ya pasó, no hay que tener en cuenta el día bisiesto
    // para el cálculo. cosa que este código lamentablemente sí hace
    fn sumar_dias(&mut self, dias: u64) {
        let mut dias = dias;

        // si es mayor o igual a un año
        while dias >= self.dias_ano_actual() as u64 {
            dias-= self.dias_ano_actual() as u64;
            self.ano += 1;
        }

        // si es mayor o igual al mes actual
        while dias >= self.dias_mes_actual() as u64 {
            dias-= self.dias_mes_actual() as u64;

            if self.mes < 12 {
                self.mes+= 1;
            } else {
                self.mes = 1;
                self.ano+= 1;
            }
        }

        // último tramo. el cast a u8 es seguro porque dias <= 31
        let mut dias = dias as u8;
        if self.dia + dias > self.dias_mes_actual() {
            dias-= self.dias_mes_actual() - self.dia;
            if self.mes < 12 {
                self.mes+= 1;
            } else {
                self.mes = 1;
                self.ano += 1;
            }

            self.dia = dias;
        } else {
            self.dia += dias;
        }
    }

    fn restar_dias(&mut self, dias: u64) {
        let mut dias = dias;

        // si es mayor o igual a un año
        while dias >= self.dias_ano_actual() as u64 {
            dias-= self.dias_ano_actual() as u64;
            self.ano -= 1;
        }

        // si es mayor o igual al mes actual
        while dias >= self.dias_mes_actual() as u64 {
            dias-= self.dias_mes_actual() as u64;

            if self.mes > 1 {
                self.mes-= 1;
            } else {
                self.mes = 12;
                self.ano-= 1;
            }
        }

        // último tramo. el cast a u8 es seguro porque dias <= 31
        let mut dias = dias as u8;
        if dias >= self.dia {
            if self.mes > 1 {
                self.mes-= 1;
            } else {
                self.mes = 12;
                self.ano-= 1;
            }

            dias-= self.dia;
            self.dia = self.dias_mes_actual() - dias;
        } else {
            self.dia -= dias;
        }
    }

    fn dias_mes_actual(&self) -> u8 {
        if (1..=12).contains(&self.mes) {
            if self.es_bisiesto() { DIAS_POR_MES_BIS[self.mes as usize - 1] }
            else { DIAS_POR_MES[self.mes as usize - 1] }
        } else { panic!("ERROR: mes inválido") }
    }

    fn dias_ano_actual(&self) -> u16 {
        if self.es_bisiesto() { 366 }
        else { 365 }
    }
}

fn main() {
    let mut fecha = Fecha::new(22, 08, 2002);
    println!("{}", fecha);
    fecha.sumar_dias(5000);
    println!("{}", fecha);
    fecha.restar_dias(5000);
    println!("{}", fecha);

    println!();

    let mut fecha2 = Fecha::new(31, 12, 2025);
    fecha2.sumar_dias(1);
    println!("{}", fecha2);
    fecha2.restar_dias(1);
    println!("{}", fecha2);
}