use csv;
use std::error::Error;
use std::fs::File;
use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Clone)]
pub struct Flower{
    pub comprimento_sepala: f64,
    pub largura_petala: f64,
    pub especie: String,
}

pub fn carregar_dataset() -> Vec<Flower> {
    vec![
        Flower {
            comprimento_sepala: 5.1,
            largura_petala: 1.4,
            especie: String::from("Espécie A"),
        },
        Flower {
            comprimento_sepala: 4.9,
            largura_petala: 1.4,
            especie: String::from("Espécie A"),
        },
        Flower {
            comprimento_sepala: 4.7,
            largura_petala: 1.3,
            especie: String::from("Espécie A"),
        },
        Flower {
            comprimento_sepala: 4.6,
            largura_petala: 1.5,
            especie: String::from("Espécie A"),
        },
        Flower {
            comprimento_sepala: 5.0,
            largura_petala: 1.4,
            especie: String::from("Espécie A"),
        },
        Flower {
            comprimento_sepala: 5.4,
            largura_petala: 1.5,
            especie: String::from("Espécie A"),
        },
        Flower {
            comprimento_sepala: 6.3,
            largura_petala: 2.5,
            especie: String::from("Espécie B"),
        },
        Flower {
            comprimento_sepala: 5.8,
            largura_petala: 2.3,
            especie: String::from("Espécie B"),
        },
        Flower {
            comprimento_sepala: 7.1,
            largura_petala: 2.5,
            especie: String::from("Espécie B"),
        },
        Flower {
            comprimento_sepala: 6.3,
            largura_petala: 1.9,
            especie: String::from("Espécie B"),
        },
        Flower {
            comprimento_sepala: 6.5,
            largura_petala: 2.1,
            especie: String::from("Espécie B"),
        },
        Flower {
            comprimento_sepala: 7.6,
            largura_petala: 2.3,
            especie: String::from("Espécie B"),
        },
    ]
}


pub fn carregar_dataset_iris() -> Result<Vec<Flower>, Box<dyn Error>>  {
    let mut reader = csv::Reader::from_path("C:/Users/leohe/OneDrive/Documentos/IA_Rust/classifier/data/iris.csv")?;

    let mut dataset: Vec<Flower> = Vec::new();

    for results in reader.records(){
        let record = results?;

        let comprimento_sepala: f64 = record[0].parse()?;
        let largura_petala: f64 = record[1].parse()?;
        let especie: String = record[4].to_string();

        let nova_flor = Flower{
            comprimento_sepala,
            largura_petala,
            especie,
        };

        dataset.push(nova_flor);
    }

    Ok(dataset)

}

pub fn embaralhar_dataset(dataset: &mut Vec<Flower>) {
    dataset.shuffle(&mut thread_rng());
}

