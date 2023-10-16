pub fn str_split<'a>(mut s: &'a str, delimiter: &'a str) -> Vec<&'a str> {
    let mut segments = Vec::new();
    while let Some(index) = s.find(delimiter) {
        segments.push(&s[..index]);
        s = &s[index + delimiter.len()..];
    }
    segments.push(s);
    segments
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn str_split_works() {
        assert_eq!(str_split("a,bc,def", ","), vec!["a", "bc", "def"]);
        assert_eq!(str_split("a,,", ","), vec!["a", "", ""]);
        assert_eq!(str_split(",a", ","), vec!["", "a"]);
    }
}
