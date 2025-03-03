use crate::ejemplos::ej_lectura;

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
    }
}

fn main() {
    let example_array = vec![vec![1,2], vec![2,3], vec![3,4]];

    let st = leetcode::daily_chal1::Solution::merge_arrays(vec![vec![1, 2], vec![2, 3]], vec![vec![3, 4], vec![4, 5]]);
}