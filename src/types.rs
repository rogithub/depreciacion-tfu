pub mod types {
    use serde::{Deserialize, Serialize};
    use std::io::{self};

    #[derive(Clone, Serialize, Deserialize, Debug)]
    #[serde(untagged)]
    pub enum Input {
        #[serde(rename_all = "camelCase")]
        TasaFijaUniforme {
            costo: f64,
            desecho: f64,
            vida_util: f64,
        },
    }

    #[derive(Clone, Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct Output {
        pub year: i64,
        pub depreciacion_anual: f64,
        pub depreciacion_acumulada: f64,
        pub valor_neto: f64,
    }

    pub fn json_to_input(line: io::Result<String>) -> Input {
        let json = &line.unwrap();
        let deserialized: Input = serde_json::from_str(json).unwrap();
        deserialized
    }

    pub fn first<T>(v: &Vec<T>) -> Option<&T> {
        v.first()
    }

    pub fn tasa_fija_uniforme(input: &Input) -> f64 {
        match input {
            Input::TasaFijaUniforme {
                costo,
                desecho,
                vida_util,
            } => 1.0 - f64::powf(desecho / costo, 1.0 / vida_util),
        }
    }

    pub fn years(input: &Input) -> f64 {
        match input {
            Input::TasaFijaUniforme { vida_util, .. } => vida_util.clone(),
        }
    }

    pub fn cost(input: &Input) -> f64 {
        match input {
            Input::TasaFijaUniforme { costo, .. } => costo.clone(),
        }
    }

    /*
    pub fn to_output(item: &Input) -> Vec<Output> {
        let vida_util = years(item) as i64 + 1;
        let mut valor_neto = cost(item);
        let mut dep_acc = 0 as f64;
        let mut result = Vec::new();
        let tfu = tasa_fija_uniforme(item);
        for year in 0..vida_util {
            if year == 0 {
                result.push(Output {
                    year: year,
                    depreciacion_anual: 0 as f64,
                    depreciacion_acumulada: 0 as f64,
                    valor_neto: valor_neto,
                });
                continue;
            }
            let dep = valor_neto * tfu;
            dep_acc = dep_acc + dep;
            valor_neto = valor_neto - dep;
            result.push(Output {
                year: year,
                depreciacion_anual: dep,
                depreciacion_acumulada: dep_acc,
                valor_neto: valor_neto,
            });
        }

        result
    }

    pub fn output_to_json(output: &Output) -> String {
        serde_json::to_string(output).unwrap()
    }
    */
}
