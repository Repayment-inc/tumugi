use super::task_executor::TaskExecutor;
use std::error::Error;

/// 簡単な計算機能を提供する構造体
pub struct Calculator;

impl TaskExecutor for Calculator {
    /// 与えられた数式を計算し、結果を文字列で返す
    fn execute(&self, input: &str) -> Result<String, Box<dyn Error>> {
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.len() != 3 {
            return Err("無効な入力形式です。「数字 演算子 数字」の形式で入力してください。".into());
        }

        let a: f64 = parts[0].parse()?;
        let b: f64 = parts[2].parse()?;
        let result = match parts[1] {
            "+" => a + b,
            "-" => a - b,
            "*" => a * b,
            "/" => {
                if b == 0.0 {
                    return Err("0による除算はできません".into());
                }
                a / b
            },
            _ => return Err("サポートされていない演算子です".into()),
        };

        Ok(result.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculator_addition() {
        let calculator = Calculator;
        assert_eq!(calculator.execute("5 + 3").unwrap(), "8");
    }

    #[test]
    fn test_calculator_division() {
        let calculator = Calculator;
        assert_eq!(calculator.execute("10 / 2").unwrap(), "5");
    }

    #[test]
    fn test_calculator_division_by_zero() {
        let calculator = Calculator;
        assert!(calculator.execute("5 / 0").is_err());
    }

    #[test]
    fn test_calculator_invalid_input() {
        let calculator = Calculator;
        assert!(calculator.execute("5 + 3 + 2").is_err());
    }
}