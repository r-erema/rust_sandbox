#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use std::fmt::{Formatter};

    #[test]
    fn test_collection_update() {
        let v1: Vec<i32> = Vec::new();
        let mut v2 = vec![1,2,3,4,5];
        v2.push(6);
        v2.push(7);
        assert_eq!(0, v1.len());
        assert_eq!(7, v2.len())
    }

    #[test]
    fn test_collection_read() {
        let mut v1 = vec![0, 2, 4, 6, 8];

        let third: &i32 = &v1[2];
        assert_eq!(4, *third);

        match v1.get(3) {
            Some(fourth) => assert_eq!(6, *fourth),
            None => assert!(false)
        }

        let mut expected_val = 0;
        for i in &v1 {
            assert_eq!(expected_val, *i);
            expected_val += 2;
        }

        for i in &mut v1 {
            *i += 2
        }
        assert_eq!(2, v1[0]);
        assert_eq!(6, v1[2]);
        assert_eq!(10, v1[4]);
    }

    #[test]
    fn test_collection_enum() {
        enum SpreadSheetCell {
            Int(i32),
            Float(f64),
            Text(String)
        }
        use core::fmt;
        impl fmt::Display for SpreadSheetCell {
            fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
                match self {
                    SpreadSheetCell::Int(number) => write!(f, "{}", number),
                    SpreadSheetCell::Float(number) => write!(f, "{}", number),
                    SpreadSheetCell::Text(text) => write!(f, "{}", text),
                }
            }
        }

        let row = vec![
            SpreadSheetCell::Int(3),
            SpreadSheetCell::Text(String::from("blue")),
            SpreadSheetCell::Float(10.12),
        ];

        assert_eq!("3", row[0].to_string());
        assert_eq!("blue", row[1].to_string());
        assert_eq!("10.12", row[2].to_string());

        match &row[0] {
            SpreadSheetCell::Int(number) => assert_eq!(3, *number),
            SpreadSheetCell::Float(_) => assert!(false),
            SpreadSheetCell::Text(_) => assert!(false)
        }
        match &row[1] {
            SpreadSheetCell::Int(_) => assert!(false),
            SpreadSheetCell::Float(_) => assert!(false),
            SpreadSheetCell::Text(text) => assert_eq!("blue", text)
        }
        match &row[2] {
            SpreadSheetCell::Int(_) => assert!(false),
            SpreadSheetCell::Float(number) => assert_eq!(10.12, *number),
            SpreadSheetCell::Text(_) => assert!(false)
        }
    }

    #[test]
    fn test_hash_map() {
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);

        match scores.get(&String::from("Blue")) {
            Some(v) => assert_eq!(10, *v),
            None => assert!(false)
        }

        match scores.get(&String::from("Yellow")) {
            Some(v) => assert_eq!(50, *v),
            None => assert!(false)
        }

        for (k, v) in scores {
            let s = format!("{}: {}", k, v);

            if k == "Blue" {
                assert_eq!("Blue: 10", s)
            }

            if k == "Yellow" {
                assert_eq!("Yellow: 50", s)
            }
        }
    }

    #[test]
    fn test_updating_hash_map() {
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Blue"), 25);
        match scores.get(&String::from("Blue")) {
            Some(v) => assert_eq!(25, *v),
            None => assert!(false)
        }

        let mut scores2 = HashMap::new();
        scores2.insert(String::from("Red"), 10);
        scores2.entry(String::from("Red")).or_insert(25);
        match scores2.get(&String::from("Red")) {
            Some(v) => assert_eq!(10, *v),
            None => assert!(false)
        }

        let text = "hello world wonderful world";
        let mut map = HashMap::new();
        for word in text.split_whitespace() {
            let count = map.entry(word).or_insert(0);
            *count += 1;
        }
        match map.get("world") {
            Some(v) => assert_eq!(2, *v),
            None => assert!(false)
        }
        match map.get("hello") {
            Some(v) => assert_eq!(1, *v),
            None => assert!(false)
        }
        match map.get("wonderful") {
            Some(v) => assert_eq!(1, *v),
            None => assert!(false)
        }
    }
}
