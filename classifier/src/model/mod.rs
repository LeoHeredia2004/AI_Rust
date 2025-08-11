use super::data::Flower;
use std::collections::HashMap;

pub fn calcular_distancia(flor1: &Flower, flor2: &Flower) -> f64{
    let distance : f64 = (flor1.comprimento_sepala - flor2.comprimento_sepala).powi(2) + (flor1.largura_petala - flor2.largura_petala).powi(2);
    distance.sqrt()
}

pub fn encontrar_k_vizinhos<'a>(nova_flor: &Flower, dataset: &'a Vec<Flower>, k: usize) -> Vec<&'a Flower> {
    let mut distancias: Vec<(f64, &'a Flower)> = Vec::new();

    for flor_treino in dataset{
        let dist = calcular_distancia(nova_flor, flor_treino);
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

pub fn avaliar_modelo(dataset: &Vec<Flower>, k: usize) -> f64{
    let mut acuracias: Vec<f64> = Vec::new();
    let mut tamanho_dobra = dataset.len() / k;

    for i in 0..k{
        let inicio_teste = i * tamanho_dobra;
        let fim_teste = inicio_teste + tamanho_dobra;

        let teste_set = &dataset[inicio_teste..fim_teste];

        let mut treino_set = Vec::new();
        // Pegar a parte do dataset antes da dobra de teste
        treino_set.extend_from_slice(&dataset[0..inicio_teste]);
        // Pegar a parte do dataset depois da dobra de teste
        treino_set.extend_from_slice(&dataset[fim_teste..]);


        let mut acertos = 0;
        let mut testados = 0;

        for flor_teste in teste_set {
            testados += 1;

            let k_vizinhos = 3;

            let vizinhos = encontrar_k_vizinhos(flor_teste, &treino_set, k_vizinhos);

            let especie_prevista = votacao(&vizinhos);

            if especie_prevista == flor_teste.especie {
                acertos += 1;
            }
        }
        let acuracia_dobra = acertos as f64 / testados as f64;
        acuracias.push(acuracia_dobra);
    }

    let soma_acuracias: f64 = acuracias.iter().sum();
    soma_acuracias / acuracias.len() as f64
}
