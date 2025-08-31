//! Módulo de regressão linear para análise de séries temporais
//! Implementação pura sem dependências externas

use std::error::Error;
use std::fmt;

/// Erros personalizados para a regressão linear
#[derive(Debug)]
enum LinearRegressionError {
    EmptyData,
    InsufficientData,
    InvalidInput,
}

impl fmt::Display for LinearRegressionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LinearRegressionError::EmptyData => write!(f, "Dados vazios fornecidos"),
            LinearRegressionError::InsufficientData => write!(f, "Dados insuficientes para análise"),
            LinearRegressionError::InvalidInput => write!(f, "Entrada inválida"),
        }
    }
}

impl Error for LinearRegressionError {}

/// Estrutura que armazena os coeficientes da regressão linear
#[derive(Debug, Clone, PartialEq)]
struct LinearCoefficients {
    slope: f64,      // Inclinação da reta (β1)
    intercept: f64,  // Intercepto (β0)
}

/// Estrutura que armazena os resultados da regressão
#[derive(Debug, Clone)]
struct RegressionResult {
    coefficients: LinearCoefficients,
    r_squared: f64,
    mse: f64,
    predictions: Vec<f64>,
}

/// Implementação da regressão linear
struct LinearRegression;

impl LinearRegression {
    /// Realiza regressão linear em uma série temporal
    fn fit(data: &[f64]) -> Result<RegressionResult, LinearRegressionError> {
        if data.is_empty() {
            return Err(LinearRegressionError::EmptyData);
        }
        
        if data.len() < 2 {
            return Err(LinearRegressionError::InsufficientData);
        }

        // Cria índices de tempo (0, 1, 2, ..., n-1)
        let x: Vec<f64> = (0..data.len()).map(|i| i as f64).collect();
        let y = data.to_vec();

        // Calcula coeficientes
        let coefficients = Self::calculate_coefficients(&x, &y)?;
        
        // Faz previsões para os dados de treino
        let predictions = Self::predict_range(&x, &coefficients);
        
        // Calcula métricas
        let r_squared = Self::calculate_r_squared(&y, &predictions);
        let mse = Self::calculate_mse(&y, &predictions);

        Ok(RegressionResult {
            coefficients,
            r_squared,
            mse,
            predictions,
        })
    }

    /// Calcula os coeficientes da regressão linear (mínimos quadrados)
    fn calculate_coefficients(x: &[f64], y: &[f64]) -> Result<LinearCoefficients, LinearRegressionError> {
        let n = x.len() as f64;
        
        // Somas necessárias para os cálculos
        let sum_x: f64 = x.iter().sum();
        let sum_y: f64 = y.iter().sum();
        let sum_xy: f64 = x.iter().zip(y.iter()).map(|(xi, yi)| xi * yi).sum();
        let sum_x2: f64 = x.iter().map(|xi| xi * xi).sum();
        
        // Denominador para evitar divisão por zero
        let denominator = n * sum_x2 - sum_x * sum_x;
        
        if denominator.abs() < f64::EPSILON {
            return Err(LinearRegressionError::InvalidInput);
        }
        
        // Cálculo da inclinação (slope)
        let slope = (n * sum_xy - sum_x * sum_y) / denominator;
        
        // Cálculo do intercepto
        let intercept = (sum_y - slope * sum_x) / n;
        
        Ok(LinearCoefficients { slope, intercept })
    }

    /// Realiza previsões para um range de valores x
    fn predict_range(x: &[f64], coefficients: &LinearCoefficients) -> Vec<f64> {
        x.iter()
            .map(|&xi| coefficients.slope * xi + coefficients.intercept)
            .collect()
    }

    /// Realiza previsões para períodos futuros
    fn forecast(coefficients: &LinearCoefficients, periods: usize) -> Vec<f64> {
        (0..periods)
            .map(|i| coefficients.slope * (i as f64) + coefficients.intercept)
            .collect()
    }

    /// Calcula o coeficiente de determinação R²
    fn calculate_r_squared(actual: &[f64], predicted: &[f64]) -> f64 {
        let mean_actual: f64 = actual.iter().sum::<f64>() / actual.len() as f64;
        let total_sum_squares: f64 = actual.iter().map(|&yi| (yi - mean_actual).powi(2)).sum();
        let residual_sum_squares: f64 = actual.iter()
            .zip(predicted.iter())
            .map(|(&yi, &y_pred)| (yi - y_pred).powi(2))
            .sum();
        
        if total_sum_squares.abs() < f64::EPSILON {
            return 1.0; // Dados constantes
        }
        
        1.0 - (residual_sum_squares / total_sum_squares)
    }

    /// Calcula o erro quadrático médio (MSE)
    fn calculate_mse(actual: &[f64], predicted: &[f64]) -> f64 {
        actual.iter()
            .zip(predicted.iter())
            .map(|(&yi, &y_pred)| (yi - y_pred).powi(2))
            .sum::<f64>() / actual.len() as f64
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("==========================================");
    println!("   TIMEWISE ANALYTICS - REGRESSÃO LINEAR");
    println!("==========================================");
    
    // Exemplo 1: Dados de vendas mensais
    println!("\n📊 EXEMPLO 1: Vendas Mensais");
    println!("------------------------------------------");
    
    let sales_data = vec![100.0, 120.0, 130.0, 145.0, 160.0];
    println!("Dados: {:?}", sales_data);
    
    let result = LinearRegression::fit(&sales_data)?;
    
    println!("Inclinação (Slope): {:.2}", result.coefficients.slope);
    println!("Intercepto: {:.2}", result.coefficients.intercept);
    println!("R²: {:.4}", result.r_squared);
    println!("MSE: {:.4}", result.mse);
    
    let forecast = LinearRegression::forecast(&result.coefficients, 3);
    println!("\n📈 Previsões para os próximos 3 períodos:");
    for (i, prediction) in forecast.iter().enumerate() {
        println!("Período {}: {:.2}", i + sales_data.len() + 1, prediction);
    }

    // Exemplo 2: Dados com tendência decrescente
    println!("\n📊 EXEMPLO 2: Tendência Decrescente");
    println!("------------------------------------------");
    
    let decreasing_data = vec![50.0, 45.0, 40.0, 35.0, 30.0];
    println!("Dados: {:?}", decreasing_data);
    
    let result2 = LinearRegression::fit(&decreasing_data)?;
    
    println!("Inclinação (Slope): {:.2}", result2.coefficients.slope);
    println!("Intercepto: {:.2}", result2.coefficients.intercept);
    println!("R²: {:.4}", result2.r_squared);
    println!("MSE: {:.4}", result2.mse);
    
    let forecast2 = LinearRegression::forecast(&result2.coefficients, 2);
    println!("\n📈 Previsões para os próximos 2 períodos:");
    for (i, prediction) in forecast2.iter().enumerate() {
        println!("Período {}: {:.2}", i + decreasing_data.len() + 1, prediction);
    }

    // Exemplo 3: Teste com dados perfeitos (y = 2x + 1)
    println!("\n📊 EXEMPLO 3: Dados Lineares Perfeitos");
    println!("------------------------------------------");
    
    let perfect_data = vec![1.0, 3.0, 5.0, 7.0, 9.0];
    println!("Dados: {:?}", perfect_data);
    println!("Relação esperada: y = 2x + 1");
    
    let result3 = LinearRegression::fit(&perfect_data)?;
    
    println!("Inclinação (Slope): {:.2}", result3.coefficients.slope);
    println!("Intercepto: {:.2}", result3.coefficients.intercept);
    println!("R²: {:.4}", result3.r_squared);
    println!("MSE: {:.4}", result3.mse);
    
    // Teste de tratamento de erros
    println!("\n⚠️  TESTE DE TRATAMENTO DE ERROS");
    println!("------------------------------------------");
    
    match LinearRegression::fit(&[]) {
        Ok(_) => println!("❌ Erro: deveria ter falhado com dados vazios"),
        Err(e) => println!("✅ Correto: {}", e),
    }
    
    match LinearRegression::fit(&[42.0]) {
        Ok(_) => println!("❌ Erro: deveria ter falhado com dados insuficientes"),
        Err(e) => println!("✅ Correto: {}", e),
    }

    println!("\n==========================================");
    println!("        ANÁLISE CONCLUÍDA ✅");
    println!("==========================================");
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_regression_basic() {
        // Dados lineares perfeitos: y = 2x + 1
        let data = vec![1.0, 3.0, 5.0, 7.0, 9.0];
        
        let result = LinearRegression::fit(&data).unwrap();
        
        assert!((result.coefficients.slope - 2.0).abs() < 1e-10);
        assert!((result.coefficients.intercept - 1.0).abs() < 1e-10);
        assert!((result.r_squared - 1.0).abs() < 1e-10);
        assert!((result.mse - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_linear_regression_constant() {
        // Dados constantes
        let data = vec![5.0, 5.0, 5.0, 5.0, 5.0];
        
        let result = LinearRegression::fit(&data).unwrap();
        
        assert!((result.coefficients.slope - 0.0).abs() < 1e-10);
        assert!((result.coefficients.intercept - 5.0).abs() < 1e-10);
        assert!((result.r_squared - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_predictions() {
        let coefficients = LinearCoefficients {
            slope: 2.0,
            intercept: 1.0,
        };
        
        let x = vec![0.0, 1.0, 2.0];
        let predictions = LinearRegression::predict_range(&x, &coefficients);
        
        assert!((predictions[0] - 1.0).abs() < 1e-10);
        assert!((predictions[1] - 3.0).abs() < 1e-10);
        assert!((predictions[2] - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_forecast() {
        let coefficients = LinearCoefficients {
            slope: 1.0,
            intercept: 10.0,
        };
        
        let forecast = LinearRegression::forecast(&coefficients, 3);
        
        assert!((forecast[0] - 10.0).abs() < 1e-10);
        assert!((forecast[1] - 11.0).abs() < 1e-10);
        assert!((forecast[2] - 12.0).abs() < 1e-10);
    }

    #[test]
    fn test_empty_data() {
        let data: Vec<f64> = vec![];
        let result = LinearRegression::fit(&data);
        
        assert!(result.is_err());
        match result.err().unwrap() {
            LinearRegressionError::EmptyData => assert!(true),
            _ => assert!(false, "Erro incorreto para dados vazios"),
        }
    }

    #[test]
    fn test_insufficient_data() {
        let data = vec![1.0];
        let result = LinearRegression::fit(&data);
        
        assert!(result.is_err());
        match result.err().unwrap() {
            LinearRegressionError::InsufficientData => assert!(true),
            _ => assert!(false, "Erro incorreto para dados insuficientes"),
        }
    }

    #[test]
    fn test_r_squared_calculation() {
        let actual = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let perfect_pred = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let bad_pred = vec![2.0, 3.0, 4.0, 5.0, 6.0];
        
        let r2_perfect = LinearRegression::calculate_r_squared(&actual, &perfect_pred);
        let r2_bad = LinearRegression::calculate_r_squared(&actual, &bad_pred);
        
        assert!((r2_perfect - 1.0).abs() < 1e-10);
        assert!(r2_bad < 1.0);
    }

    #[test]
    fn test_mse_calculation() {
        let actual = vec![1.0, 2.0, 3.0];
        let predicted = vec![1.0, 2.0, 3.0];
        let bad_predicted = vec![2.0, 3.0, 4.0];
        
        let mse_perfect = LinearRegression::calculate_mse(&actual, &predicted);
        let mse_bad = LinearRegression::calculate_mse(&actual, &bad_predicted);
        
        assert!((mse_perfect - 0.0).abs() < 1e-10);
        assert!((mse_bad - 1.0).abs() < 1e-10);
    }
}