
use crate::types::types::depreciacion;
use crate::types::types::first;
use crate::types::types::json_to_input;
use crate::types::types::tasa_fija_uniforme;
use crate::types::types::Input;
use std::io::{self, BufRead};
mod types;

fn main() {
    let stdin = io::stdin();
    let inputs: Vec<Input> = stdin.lock().lines().map(json_to_input).collect();
    let tfu = tasa_fija_uniforme(first(&inputs).unwrap());
    let d = depreciacion(first(&inputs).unwrap());
    for _i in inputs {
        println!("{:?} {:?}", tfu, d);
    }
}
