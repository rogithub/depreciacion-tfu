
use crate::types::types::cost;
use crate::types::types::years;
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
    let item = first(&inputs).unwrap();
    let tfu = tasa_fija_uniforme(item);
    let d = depreciacion(item);
    let vida_util = years(item) as i64;

    println!("{:?}% ${:?}", tfu, d);
    println!("================================================================");

    let mut costo_inicial = cost(item);
    let mut dep_acc = 0 as f64;
    for year in 0..vida_util {
        let dep = costo_inicial * tfu;
        let dep = costo_inicial * tfu;
        println!("{:?} {:?} {:?} {:?}", year, dep);


    }
}
