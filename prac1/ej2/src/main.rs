fn main() {
    let mi_var: u32 = 15*16*16*16; // 61440
    let mi_var_hex = u32_to_hex(mi_var); // F000

    println!("MiVar: {mi_var}, MiVarHex: {mi_var_hex}");
}

fn u32_to_hex(num: u32) -> String {
    // convert num into hex string
    format!("{:X}", num)
}