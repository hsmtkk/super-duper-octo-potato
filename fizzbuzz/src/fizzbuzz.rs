pub fn fizzbuzz(n:i64) -> String {
    if n % 15 == 0 {
        return "FizzBuzz".to_string();
    } else if n % 5 == 0 {
        return "Buzz".to_string();
    } else if n % 3 == 0 {
        return "Fizz".to_string();
    } else {
        return n.to_string();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_fizzbuzz(){
        assert_eq!("1", super::fizzbuzz(1));
        assert_eq!("Fizz", super::fizzbuzz(3));
        assert_eq!("Buzz", super::fizzbuzz(5));
        assert_eq!("FizzBuzz", super::fizzbuzz(15));
    }
}