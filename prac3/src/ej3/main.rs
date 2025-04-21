
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

const NOMBRE_MESES: [&str; 12] = ["Enero", "Febrero", "Marzo", "Abril",
                                  "Mayo", "Junio", "Julio", "Agosto",
                                  "Septiembre", "Octubre", "Noviembre", "Diciembre"];
#[derive(Debug, PartialEq)]
struct Fecha {
    dia: u8,
    mes: u8,
    ano: i64
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
    fn new(dia: u8, mes: u8, ano: i64) -> Fecha {
        Fecha { dia, mes, ano }
    }

    fn es_fecha_valida(&self) -> bool {
        // check que el mes sea válido
        if !(1..=12).contains(&self.mes) { return false }

        // check días del mes
        if self.dia < 1 
        || self.dia > self.dias_mes_actual()
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
    fn sumar_dias(&mut self, dias: u32) {
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

    fn restar_dias(&mut self, dias: u32) {
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

    fn dias_mes_actual(&self) -> u8 {
        match self.mes {
            4 | 6 | 9 | 11 => 30,
            2 => if self.es_bisiesto() { 29 } else { 28 },
            _ => 31,
        }
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

#[cfg(test)]
mod tests {
    use crate::Fecha;

    #[test]
    fn test_bisiestos() {
        let fecha = Fecha::new(22, 8, 2002);
        assert!(!fecha.es_bisiesto(), "2002 no es un año bisiesto");
        
        let fecha_2 = Fecha::new(22, 8, 2020);
        assert!(fecha_2.es_bisiesto(), "2020 es bisiesto");
        
        let fecha_3 = Fecha::new(1, 1, 0);
        assert!(fecha_3.es_bisiesto(), "0 es bisiesto");
    }
    
    #[test]
    fn test_suma_resta() {
        let mut fecha_1 = Fecha::new(22, 8, 2002);
        let fecha_2 = Fecha::new(22, 8, 2002);
        
        fecha_1.sumar_dias(5000);
        
        assert_eq!(fecha_1, Fecha::new(30, 4, 2016), "22/08/2002 + 5000 días = 30/04/2016");
        
        fecha_1.restar_dias(5000);
        
        assert_eq!(fecha_1, fecha_2, "22/08/2002 + 5000 - 5000 == 22/08/2002");
    }
}