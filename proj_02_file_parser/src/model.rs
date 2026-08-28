// model.rs — record type + CSV parsing, JSON output, and a summary.
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;

// One CSV row maps to one Person. serde handles the column<->field mapping.
#[derive(Debug, Serialize, Deserialize)]
pub struct Person {
    pub name: String,
    pub age: u32,
    pub city: String,
}

// Read all rows from a CSV file into a Vec<Person>.
pub fn read_csv(path: &Path) -> Result<Vec<Person>> {
    let mut reader = csv::Reader::from_path(path)
        .with_context(|| format!("opening {}", path.display()))?;

    let mut people = Vec::new();
    for (i, row) in reader.deserialize().enumerate() {
        // Each row is a Result; add context so a bad row reports its line.
        let person: Person = row.with_context(|| format!("parsing row {}", i + 1))?;
        people.push(person);
    }
    Ok(people)
}

// Serialize the records to pretty JSON (shows serde across two formats).
pub fn to_json(people: &[Person]) -> Result<String> {
    serde_json::to_string_pretty(people).context("serializing to JSON")
}

#[derive(Debug)]
pub struct Summary {
    pub count: usize,
    pub avg_age: f64,
    pub oldest: Option<String>,
}

// Compute a small summary using iterators.
pub fn summarize(people: &[Person]) -> Summary {
    let count = people.len();
    let sum_age: u32 = people.iter().map(|p| p.age).sum();
    let avg_age = if count == 0 { 0.0 } else { sum_age as f64 / count as f64 };
    let oldest = people
        .iter()
        .max_by_key(|p| p.age)
        .map(|p| p.name.clone());
    Summary { count, avg_age, oldest }
}
