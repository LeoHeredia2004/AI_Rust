
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