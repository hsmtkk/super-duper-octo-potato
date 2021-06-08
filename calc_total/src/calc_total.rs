pub trait Calculator {
    fn calculate(&self, price:i64) -> i64;
}

struct CalculatorImpl {}

fn new() -> impl Calculator {
    return CalculatorImpl::new();
}

impl CalculatorImpl {
    fn new() -> CalculatorImpl {
        return CalculatorImpl{};
    }
}

impl Calculator for CalculatorImpl {
    fn calculate(&self, price:i64) -> i64 {
        let mut total = (price as f64) * 1.1;
        if total < 2000.0 {
            total += 350.0;
        }
        return total as i64;
    }
}

#[cfg(test)]
mod tests {
    use super::Calculator;
    #[test]
    fn test_calculate(){
        let c = super::new();
        let want = 900;
        let got = c.calculate(500);
        assert_eq!(want, got);
        let want = 3300;
        let got = c.calculate(3000);
        assert_eq!(want, got);
    }
}