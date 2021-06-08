fn average(scores: Vec<i64>) -> i64 {
    let mut sum = 0;
    for s in &scores {
        sum += s;
    }
    return (sum as f64 / scores.len() as f64) as i64;
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_average() {
        let scores = vec![10, 16, 20, 21, 68];
        let want = 27;
        let got = super::average(scores);
        assert_eq!(want, got);
    }
}