use std::collections::BTreeMap;
use std::io;

/// Zamienia wejściowy tekst na wektor znaków zawierający tylko litery alfabetu.
/// Zwraca `Err("Brak danych")`, gdy po przetworzeniu brak liter.
pub fn normalize_letters(input: &str) -> Result<Vec<char>, String> {
    let letters: Vec<char> = input
        .trim()
        .chars()
        .map(|c| c.to_ascii_lowercase())
        .filter(|c| c.is_ascii_alphabetic())
        .collect();

    if letters.is_empty() {
        return Err(String::from("Brak danych"));
    }

    Ok(letters)
}

/// Buduje histogram wystąpień liter na podstawie wektora znaków.
pub fn build_histogram(letters: &[char]) -> BTreeMap<char, usize> {
    let mut counts = BTreeMap::new();

    for &ch in letters {
        counts.entry(ch).and_modify(|c| *c += 1).or_insert(1);
    }

    counts
}

/// Formatuje histogram do wypisania na stdout jako linie `litera: liczba`.
pub fn format_histogram(counts: &BTreeMap<char, usize>) -> Vec<String> {
    let mut lines = Vec::new();
    for (letter, count) in counts {
        lines.push(format!("{letter}: {count}"));
    }

    lines
}

/// Główna logika: przygotowuje wynik do wypisania lub zwraca komunikat błędu.
pub fn run_from_str(input: &str) -> Result<Vec<String>, String> {
    match normalize_letters(input) {
        Ok(letters) => {
            let histogram = build_histogram(&letters);
            Ok(format_histogram(&histogram))
        }
        Err(e) => Err(e),
    }
}

pub fn main() {
    let mut buffer = String::new();

    if io::stdin().read_line(&mut buffer).is_err() {
        println!("Brak danych");
        return;
    }

    match run_from_str(&buffer) {
        Ok(lines) => {
            for line in lines {
                println!("{}", line);
            }
        }
        Err(message) => println!("{}", message),
    }
}
