use super::data::Flower;
use std::collections::HashMap;

pub fn calcular_distancia(flor1: &Flower, flor2: &Flower) -> f64{
    let distance : f64 = (flor1.comprimento_sepala - flor2.comprimento_sepala).powi(2) + (flor1.largura_petala - flor2.largura_petala).powi(2);
    distance.sqrt()
}

pub fn encontrar_k_vizinhos<'a>(nov_flor: &Flower, dataset: &'a Vec<Flower>, k: usize) -> Vec<&'a Flower> {
    let mut distancias: Vec<(f64, &'a Flower)> = Vec::new();

    for flor_treino in dataset{
        let dist = calcular_distancia(nov_flor, flor_treino);
        distancias.push((dist, flor_treino));
    }

    //ordenar
    distancias.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    distancias.into_iter()
        .take(k)
        .map(|(_, flor)| flor)
        .collect()
}

pub fn votacao(vizinhos: &[&Flower]) -> String{
    let mut contagem:HashMap<String, usize> = HashMap::new();

    for flor in vizinhos{
        *contagem.entry(flor.especie.clone()).or_insert(0) += 1;
    }

    let (especie_vencedora, _) = contagem.into_iter()
        .max_by(|a, b| a.1.cmp(&b.1))
        .unwrap();

    especie_vencedora.to_string()
}

