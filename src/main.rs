use crate::ejemplos::ej_lectura;
use crate::practicas::prac1::{prac1_ej1, prac1_ej9};

mod ejemplos {
    pub mod ej_shadowing;
    pub mod ej_divs;
    pub mod ej_lectura;
}

mod leetcode {
    pub mod daily_chal1;
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
        pub mod prac1_ej9;
        pub mod prac1_ej10;
    }
}

fn main() {
    prac1_ej9::ej9();
}