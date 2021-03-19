use crate::types::types::cost;
use crate::types::types::first;
use crate::types::types::json_to_input;
use crate::types::types::tasa_fija_uniforme;
use crate::types::types::years;
use crate::types::types::Input;
use std::io::{self, BufRead};
mod types;

fn main() {
    let stdin = io::stdin();
    let inputs: Vec<Input> = stdin.lock().lines().map(json_to_input).collect();
    let item = first(&inputs).unwrap();
    let tfu = tasa_fija_uniforme(item);
    let vida_util = years(item) as i64 + 1;

    println!(
        "===================================================================================="
    );
    println!("Tasa de depreaciacón {:?}%", tfu);
    println!(
        "===================================================================================="
    );
    println!("Año \t\t Depreciación \t\t Depreciación \t\t Valor Neto");
    println!("Año \t\t Anual        \t\t Acumulada    \t\t ");
    println!(
        "===================================================================================="
    );

    let mut valor_neto = cost(item);
    let mut dep_acc = 0 as f64;
    for year in 0..vida_util {
        if year == 0 {
            println!(
                "{:?} \t\t {:?} \t\t\t\t {:?} \t\t\t\t {:?}",
                0, 0, 0, valor_neto
            );
            continue;
        }
        let dep = valor_neto * tfu;
        dep_acc = dep_acc + dep;
        valor_neto = valor_neto - dep;
        println!(
            "{:?} \t\t {:?} \t\t {:?} \t\t {:?}",
            year, dep, dep_acc, valor_neto
        );
    }
}
