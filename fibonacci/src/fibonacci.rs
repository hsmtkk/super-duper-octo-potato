use std::collections::HashMap;

pub trait Calculator {
    fn fibonacci(&mut self, n:i128) -> i128;
}

struct CalculatorImpl {
    memo: HashMap<i128, i128>,
}

fn new() -> impl Calculator {
    return CalculatorImpl::new();
}

impl CalculatorImpl {
    fn new() -> CalculatorImpl {
        return CalculatorImpl{memo:HashMap::new()};
    }
}

impl Calculator for CalculatorImpl {
    fn fibonacci(&mut self, n:i128) -> i128{
        if n <= 2 {
            return 1;
        }
        if let Some(val) = self.memo.get(&n) {
            return *val;
        } else {
            let result = self.fibonacci(n-1) + self.fibonacci(n-2);
            self.memo.insert(n, result);
            return result;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Calculator;
    #[test]
    fn test_fibonacci(){
        let want = 354224848179261915075;
        let got = super::new().fibonacci(100);
        assert_eq!(want, got);
    }
}