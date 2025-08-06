// src/main.rs

mod data;
mod model;

use data::Flower;
use model::{calcular_distancia, encontrar_k_vizinhos, votacao};

fn main() {
    println!("Projeto classificador de flores!");

    // Passo 1: Carregar o dataset
    let dataset: Vec<Flower> = data::carregar_dataset();
    println!("Total de Floweres no dataset: {}", dataset.len());

    // --- Passo 2: Criar uma nova flor para classificar ---
    let nova_flor = Flower {
        comprimento_sepala: 5.9,
        largura_petala: 1.8,
        especie: String::from("???"), // A espécie que queremos prever!
    };

    println!("\nClassificando a nova flor:");
    println!(
        "Sepala={:.1}, Petala={:.1}, Especie={}",
        nova_flor.comprimento_sepala, nova_flor.largura_petala, nova_flor.especie
    );

    // --- Passo 3: Definir k ---
    let k = 3;

    // --- Passo 4: Encontrar os k vizinhos mais próximos ---
    let vizinhos = encontrar_k_vizinhos(&nova_flor, &dataset, k);

    println!("\nOs {} vizinhos mais próximos são:", k);
    for vizinho in &vizinhos {
        println!(
            "-> Sepala={:.1}, Petala={:.1}, Especie={}",
            vizinho.comprimento_sepala, vizinho.largura_petala, vizinho.especie
        );
    }

    // --- Passo 5: Votar para prever a espécie ---
    let especie_prevista = votacao(&vizinhos);

    // --- Passo 6: Imprimir o resultado ---
    println!(
        "\nO classificador k-NN previu que a nova flor pertence à: {}",
        especie_prevista
    );
}