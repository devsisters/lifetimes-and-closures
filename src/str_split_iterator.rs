pub struct StrSplit<'s, 'delim> {
    remainder: Option<&'s str>,
    delimiter: &'delim str,
}

pub fn str_split<'s, 'delim>(s: &'s str, delimiter: &'delim str) -> StrSplit<'s, 'delim> {
    StrSplit {
        remainder: Some(s),
        delimiter,
    }
}

impl<'s> Iterator for StrSplit<'s, '_> {
    type Item = &'s str;

    fn next(&mut self) -> Option<&'s str> {
        let remainder = self.remainder.as_mut()?;
        if let Some(index) = remainder.find(self.delimiter) {
            let segment = &remainder[..index];
            *remainder = &remainder[index + self.delimiter.len()..];
            Some(segment)
        } else {
            self.remainder.take()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn str_split_works() {
        assert_eq!(
            str_split("a,bc,def", ",").collect::<Vec<_>>(),
            vec!["a", "bc", "def"],
        );
        assert_eq!(str_split("a,,", ",").collect::<Vec<_>>(), vec!["a", "", ""]);
        assert_eq!(str_split(",a", ",").collect::<Vec<_>>(), vec!["", "a"]);
    }
}
