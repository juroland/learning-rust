use std::{
    fs::File,
    io::{self, BufRead},
};

pub struct CSVReader {
    delimiter: char,
    lines: io::Lines<io::BufReader<File>>,
}

impl CSVReader {
    pub fn new(file: File, delimiter: char) -> Self {
        CSVReader {
            delimiter,
            lines: io::BufReader::new(file).lines(),
        }
    }
}

impl Iterator for CSVReader {
    type Item = io::Result<Vec<String>>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(Ok(line)) = self.lines.next() {
            let values: Vec<String> = line
                .split(self.delimiter as char)
                .map(|s| s.to_string())
                .collect();
            Some(Ok(values))
        } else {
            None
        }
    }
}
