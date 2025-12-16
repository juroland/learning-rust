use learning_rust::csv;
use std::io::{self, Seek, Write};
use tempfile::tempfile;

fn read_row(reader: &mut csv::CSVReader) -> Vec<String> {
    reader
        .next()
        .expect("Expected another row")
        .expect("Expected successful read")
}

#[test]
fn test_csv_reader() -> io::Result<()> {
    let csv_lines = vec!["name,age,city", "Alice,30,New York", "Bob,25,Los Angeles"];

    let mut file = tempfile()?;
    file.write_all(csv_lines.join("\n").as_bytes())?;
    file.rewind()?;

    let mut reader = csv::CSVReader::new(file, ',');

    assert_eq!(read_row(&mut reader), vec!["name", "age", "city"]);
    assert_eq!(read_row(&mut reader), vec!["Alice", "30", "New York"]);
    assert_eq!(read_row(&mut reader), vec!["Bob", "25", "Los Angeles"]);
    assert!(reader.next().is_none());

    Ok(())
}
