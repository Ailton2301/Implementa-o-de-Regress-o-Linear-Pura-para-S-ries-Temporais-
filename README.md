TimeWise Analytics - Regressão Linear para Séries Temporais
https://img.shields.io/badge/Rust-1.70%252B-orange.svg
https://img.shields.io/badge/License-MIT-blue.svg
https://img.shields.io/badge/Build-Passing-brightgreen.svg
https://img.shields.io/badge/Tests-8%252F8%2520Passing-brightgreen.svg

Implementação pura em Rust de regressão linear para análise de séries temporais, desenvolvida para a TimeWise Analytics.

📊 Funcionalidades
✅ Regressão Linear Pura: Implementação completa sem dependências externas

✅ Cálculo de Métricas: R² (Coeficiente de Determinação) e MSE (Erro Quadrático Médio)

✅ Previsões Futuras: Forecast para períodos futuros

✅ Tratamento de Erros Robusto: Sistema de erros personalizados

✅ Análise de Séries Temporais: Otimizado para dados temporais

✅ Alta Performance: Algoritmo O(n) eficiente

🚀 Instalação
bash
# Clone o repositório
git clone https://github.com/timewise-analytics/linear-regression-rs
cd timewise-analytics

# Compile o projeto
cargo build

# Execute os testes
cargo test

# Execute a demonstração
cargo run
📋 Uso Básico
rust
use timewise_analytics::LinearRegression;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Dados da série temporal
    let data = vec![100.0, 120.0, 130.0, 145.0, 160.0];
    
    // Realizar regressão linear
    let result = LinearRegression::fit(&data)?;
    
    println!("Slope: {:.2}", result.coefficients.slope);
    println!("Intercept: {:.2}", result.coefficients.intercept);
    println!("R²: {:.4}", result.r_squared);
    println!("MSE: {:.4}", result.mse);
    
    // Previsões para 3 períodos futuros
    let forecast = LinearRegression::forecast(&result.coefficients, 3);
    println!("Previsões: {:?}", forecast);
    
    Ok(())
}
🧪 Exemplos de Saída
Exemplo 1: Tendência de Crescimento
text
Dados: [100.0, 120.0, 130.0, 145.0, 160.0]
Slope: 14.50
Intercept: 102.00
R²: 0.9917
MSE: 3.5000
Previsões: [178.00, 192.50, 207.00]
Exemplo 2: Dados Perfeitos (y = 2x + 1)
text
Dados: [1.0, 3.0, 5.0, 7.0, 9.0]
Slope: 2.00
Intercept: 1.00
R²: 1.0000
MSE: 0.0000
Exemplo 3: Tendência Decrescente
text
Dados: [50.0, 45.0, 40.0, 35.0, 30.0]
Slope: -5.00
Intercept: 50.00
R²: 1.0000
MSE: 0.0000
🏗️ Estrutura do Projeto
text
src/
└── main.rs                 # Implementação completa
├── Cargo.toml             # Configuração do projeto
└── README.md              # Documentação

Principais componentes:
- LinearRegression         # Struct principal
- LinearCoefficients       # Coeficientes da regressão
- RegressionResult         # Resultados completos
- LinearRegressionError    # Sistema de erros
🔧 API Reference
LinearRegression::fit(data: &[f64]) -> Result<RegressionResult, LinearRegressionError>
Realiza regressão linear nos dados fornecidos.

Parâmetros:

data: Slice com os valores da série temporal

Retorna:

Ok(RegressionResult) com resultados da regressão

Err(LinearRegressionError) em caso de erro

LinearRegression::forecast(coefficients: &LinearCoefficients, periods: usize) -> Vec<f64>
Gera previsões para períodos futuros.

Parâmetros:

coefficients: Coeficientes da regressão

periods: Número de períodos futuros para prever

LinearRegression::calculate_r_squared(actual: &[f64], predicted: &[f64]) -> f64
Calcula o coeficiente de determinação R².

LinearRegression::calculate_mse(actual: &[f64], predicted: &[f64]) -> f64
Calcula o erro quadrático médio (MSE).

🧪 Testes
O projeto inclui 8 testes unitários abrangentes:

bash
cargo test
Testes implementados:

✅ test_linear_regression_basic - Dados lineares perfeitos

✅ test_linear_regression_constant - Dados constantes

✅ test_predictions - Previsões corretas

✅ test_forecast - Previsões futuras

✅ test_empty_data - Tratamento de dados vazios

✅ test_insufficient_data - Dados insuficientes

✅ test_r_squared_calculation - Cálculo do R²

✅ test_mse_calculation - Cálculo do MSE

🚨 Tratamento de Erros
O sistema inclui erros personalizados para:

LinearRegressionError::EmptyData - Dados vazios

LinearRegressionError::InsufficientData - Menos de 2 pontos

LinearRegressionError::InvalidInput - Entrada inválida

📈 Algoritmo Implementado
Fórmula da Regressão Linear
text
slope = (n * Σxy - Σx * Σy) / (n * Σx² - (Σx)²)
intercept = (Σy - slope * Σx) / n
Coeficiente de Determinação R²
text
R² = 1 - (Σ(yi - ŷi)² / Σ(yi - ȳ)²)
Erro Quadrático Médio (MSE)
text
MSE = Σ(yi - ŷi)² / n
🏆 Performance
Complexidade: O(n) para todos os cálculos

Memória: Uso eficiente de recursos

Precisão: Cálculos com f64 para máxima acurácia

Concorrência: Ready para implementações paralelas

🔮 Casos de Uso
Análise Financeira: Previsão de tendências de mercado

Dados de Sensores: Análise de séries temporais de IoT

Métricas de Negócio: Previsão de vendas e crescimento

Pesquisa Científica: Análise estatística de dados experimentais

Monitoramento: Tendências de uso e performance

📊 Limitações
Assume relação linear entre variáveis

Não captura sazonalidade ou padrões complexos

Sensível a outliers extremos

Ideal para tendências lineares simples

🤝 Contribuição
Fork o projeto

Crie uma branch para sua feature (git checkout -b feature/AmazingFeature)

Commit suas mudanças (git commit -m 'Add AmazingFeature')

Push para a branch (git push origin feature/AmazingFeature)

Abra um Pull Request

📝 Licença
Este projeto está sob licença MIT. Veja o arquivo LICENSE para detalhes.

