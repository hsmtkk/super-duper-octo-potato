use std::collections::HashMap;

pub fn count_char(text:&str) -> HashMap<String, i64>{
    let mut result: HashMap<String, i64> = HashMap::new();
    for ch in text.chars() {
        let key: String = ch.to_string();
        if key == "\n" {
            continue;
        }
        if let Some(count) = result.get(&key) {
            result.insert(key, count + 1);
        } else {
            result.insert(key, 1);
        }
    }
    return result;
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    #[test]
    fn test_count_char(){
        let text = std::fs::read_to_string("./src/example.txt").unwrap();
        let mut want: HashMap<String, i64> = HashMap::new();
        want.insert("a".to_string(), 4);
        want.insert("b".to_string(), 1);
        want.insert("c".to_string(), 1);
        want.insert("e".to_string(), 1);
        want.insert("h".to_string(), 2);
        want.insert("i".to_string(), 1);
        want.insert("l".to_string(), 2);
        want.insert("o".to_string(), 1);
        want.insert("p".to_string(), 1);
        want.insert("r".to_string(), 2);
        want.insert("v".to_string(), 1);
        let got = super::count_char(&text);
        assert_eq!(want, got);
    }
}