struct CSVIterator<'a> {
    offset: usize,
    string: &'a str,
}

impl<'a> CSVIterator<'a> {
    fn new(string: &'a str) -> Self {
        Self { offset: 0, string }
    }
}

impl<'a> Iterator for CSVIterator<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        const DELIMITER: char = ',';

        if let Some(next_offset) = self.string.find(DELIMITER) {
            let result = &self.string[..next_offset];
            self.string = &self.string[(next_offset + DELIMITER.len_utf8())..];
            return Some(result);
        }

        if self.string.len() > 0 {
            let result = self.string;
            let length = self.string.len() - 1;
            self.string = &self.string[length..length];
            return Some(result);
        }

        None
    }
}

trait CSV<'a> {
    fn into_csv(self) -> CSVIterator<'a>;
}

impl<'a> CSV<'a> for &'a str {
    fn into_csv(self) -> CSVIterator<'a> {
        CSVIterator::new(self)
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use crate::CSV;

    #[test]
    fn parse_csv() {
        let sample_csv = ",thīs,,îs,á,čśv,,";
        let mut csv_iterator = sample_csv.into_csv();

        assert_eq!(csv_iterator.next(), Some(""));
        assert_eq!(csv_iterator.next(), Some("thīs"));
        assert_eq!(csv_iterator.next(), Some(""));
        assert_eq!(csv_iterator.next(), Some("îs"));
        assert_eq!(csv_iterator.next(), Some("á"));
        assert_eq!(csv_iterator.next(), Some("čśv"));
        assert_eq!(csv_iterator.next(), Some(""));
        assert_eq!(csv_iterator.next(), None);
    }
}
