#[cfg(test)]
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i]
        }
    }

    &s[..]
}

#[cfg(test)]
mod test {
    use crate::memory_management::example1::slices::first_word;

    #[test]
    fn test_slice() {
        let s = String::from("Lorem ipsum dolor sit amet");
        assert_eq!("Lorem", first_word(&s));
    }
}
