use crate::ejemplos::ej_lectura;

mod ejemplos {
    pub mod ej_shadowing;
    pub mod ej_divs;
    pub mod ej_lectura;
}

mod practicas {
    pub mod prac1 {
        pub mod prac1_ej1;
        pub mod prac1_ej2;
        pub mod prac1_ej3;
        pub mod prac1_ej4;
        pub mod prac1_ej5;
        pub mod prac1_ej6;
        pub mod prac1_ej7;
        pub mod prac1_ej8;
    }
}

fn main() {
    practicas::prac1::prac1_ej8::ej8()
}