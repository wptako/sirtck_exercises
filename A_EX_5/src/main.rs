use std::io;

/// Parsuje liczby całkowite rozdzielone białymi znakami.
/// Zwraca komunikat `"Brak liczb do przeanalizowania."`, jeśli po przetworzeniu nie ma żadnych wartości.
pub fn parse_numbers(input: &str) -> Result<Vec<i32>, String> {
    let mut numbers = Vec::new();

    for token in input.split_whitespace() {
        match token.parse::<i32>() {
            Ok(number) => numbers.push(number),
            Err(_) => return Err(format!("Niepoprawna liczba: {}", token)),
        }
    }

    if numbers.is_empty() {
        return Err("Brak liczb do przeanalizowania.".to_string());
    }

    Ok(numbers)
}

/// Zwraca krotkę (liczba elementów, minimum, maksimum, suma) dla przekazanych liczb.
pub fn summarize_numbers(numbers: &[i32]) -> (usize, i32, i32, i32) {
    let min = numbers.iter().copied().min().unwrap();
    let max = numbers.iter().copied().max().unwrap();
    let sum = numbers.iter().sum();
    (numbers.len(), min, max, sum)
}

/// Buduje cztery linie raportu na podstawie przekazanych liczb.
pub fn describe_numbers(numbers: &[i32]) -> Vec<String> {
    let (count, min_value, max_value, total) = summarize_numbers(numbers);

    let joined = numbers
        .iter()
        .map(|number| number.to_string())
        .collect::<Vec<_>>()
        .join(" ");

    vec![
        format!("Liczby ({}): {}", count, joined),
        format!("Minimum: {}", min_value),
        format!("Maksimum: {}", max_value),
        format!("Suma: {}", total),
    ]
}

/// Odpowiada za pełną analizę: parsowanie wejścia i przygotowanie raportu.
pub fn run_analysis(line: &str) -> Result<Vec<String>, String> {
    match parse_numbers(line) {
        Ok(numbers) => Ok(describe_numbers(&numbers)),
        Err(err) => Err(err),
    }
}

pub fn main() {
    let mut buffer = String::new();

    if io::stdin().read_line(&mut buffer).is_err() {
        println!("Nie udało się odczytać danych.");
        return;
    }

    match run_analysis(buffer.as_str()) {
        Ok(report) => {
            for line in report {
                println!("{line}");
            }
        }
        Err(message) => println!("{message}"),
    }
}
