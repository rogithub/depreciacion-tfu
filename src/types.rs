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
}
