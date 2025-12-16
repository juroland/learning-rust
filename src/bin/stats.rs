use anyhow::{Context, Result, anyhow};
use clap::Parser;
use std::vec;

#[derive(Parser)]
#[command(about = "Calculate statistics from CSV files", long_about = None)]
struct Cli {
    /// Path to the CSV file
    path: std::path::PathBuf,
}

fn format_option_value<T>(value: Option<T>) -> String
where
    T: std::fmt::Display,
{
    match value {
        Some(v) => format!("{}", v),
        None => "N/A".to_string(),
    }
}

use learning_rust::{csv, stats};
fn main() -> Result<()> {
    let args = Cli::parse();
    let csv_file = std::fs::File::open(&args.path).context("Failed to open CSV file")?;
    let mut reader = csv::CSVReader::new(csv_file, ',');

    let header = reader
        .next()
        .ok_or_else(|| anyhow!("CSV file is empty"))?
        .context("Failed to read header")?;

    let mut data = vec![vec![]; header.len()];
    for row in reader {
        let row = row?;
        for (i, value) in row.iter().enumerate() {
            data[i].push(value.parse::<f64>().unwrap_or(0.0));
        }
    }

    for (i, column) in header.iter().enumerate() {
        let col_data = &data[i];
        println!("Statistics for column '{}':", column);
        println!("  Min: {}", format_option_value(stats::min(col_data)));
        println!("  Max: {}", format_option_value(stats::max(col_data)));
        println!(
            "  Average: {}",
            format_option_value(stats::average(col_data))
        );
        println!("  Median: {}", format_option_value(stats::median(col_data)));
    }

    Ok(())
}
