// src/main.rs

mod data;
mod model;

use data::Flower;
use model::{calcular_distancia, encontrar_k_vizinhos, votacao};
use crate::data::embaralhar_dataset;


fn main() {
    println!("Projeto classificador de flores!");

    let mut dataset = data::carregar_dataset_iris()
        .expect("Falha ao carregar o dataset Iris!");
    
    // Embaralhar o dataset antes de avaliar
    data::embaralhar_dataset(&mut dataset);

    let k_folds = 5; // Vamos usar 5 dobras para a validação cruzada
    let acuracia_media = model::avaliar_modelo(&dataset, k_folds);

    println!(
        "\nAcurácia do modelo k-NN usando validação cruzada com {} dobras: {:.2}%",
        k_folds,
        acuracia_media * 100.0
    );
}