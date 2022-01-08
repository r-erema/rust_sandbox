#[cfg(test)]
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

#[cfg(test)]
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

#[cfg(test)]
mod test {
    use crate::generics::example1::generics::{largest, longest};

    #[test]
    fn test_largest() {
        assert_eq!(100, largest(&vec![34, 50, 25, 100, 65]));
    }
    #[test]
    fn test_longest() {
        let s1 = String::from("cats");
        let s2 = String::from("cat");
        let s2ref = s2.as_str();

        {
            let s1ref = s1.as_str();
            assert_eq!("cats", longest(s1ref, s2ref));
        }

    }

}
