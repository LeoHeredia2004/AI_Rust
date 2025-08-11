# IA_Rust
Classificador de Flores (k-NN)
Este projeto é uma implementação de um classificador de aprendizado de máquina usando o algoritmo k-Nearest Neighbors (k-NN), construído do zero em Rust. O objetivo é demonstrar como um algoritmo de classificação pode ser implementado em uma linguagem de sistema de forma segura e eficiente, sem a dependência de bibliotecas de machine learning de alto nível para o núcleo do modelo.

Funcionalidades
Implementação Pura de k-NN: O algoritmo k-NN, incluindo o cálculo de distância euclidiana, a busca pelos vizinhos mais próximos e a votação, foi codificado manualmente para fins didáticos.

Leitura de Dados: O projeto lê o famoso dataset de flores Iris a partir de um arquivo CSV, demonstrando como carregar dados externos em Rust.

Avaliação do Modelo: Acurácia do modelo é avaliada usando a técnica de validação cruzada k-fold, garantindo uma medição de desempenho mais robusta e confiável.

Como Executar
Clone este repositório.

Certifique-se de ter o Rust e o Cargo instalados.

Navegue até a pasta do projeto no terminal.

Execute o projeto com cargo run.


# Flower Classifier (k-NN)
This project is an implementation of a machine learning classifier using the k-Nearest Neighbors (k-NN) algorithm, built from scratch in Rust. The goal is to demonstrate how a classification algorithm can be implemented in a systems language in a safe and efficient manner, without relying on high-level machine learning libraries for the core model.

Features
Pure k-NN Implementation: The k-NN algorithm, including Euclidean distance calculation, finding nearest neighbors, and voting, was manually coded for educational purposes.

Data Reading: The project reads the famous Iris flower dataset from a CSV file, showcasing how to load external data in Rust.

Model Evaluation: The model's accuracy is assessed using the k-fold cross-validation technique, ensuring a more robust and reliable performance measurement.

How to Run
Clone this repository.

Make sure you have Rust and Cargo installed.

Navigate to the project directory in your terminal.

Run the project with cargo run.
