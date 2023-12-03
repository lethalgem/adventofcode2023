use cond_utils::Between;
use std::{fs, io, num::ParseIntError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Day3Error {
    #[error("File not loaded")]
    UnableToLoadFile(#[from] io::Error),
    #[error("Failed to parse int from string")]
    ParseIntFailed(#[from] ParseIntError),
}

#[derive(PartialEq, Debug, Clone)]
struct SchematicSymbol {
    line: usize,
    index: usize,
}

#[derive(PartialEq, Debug)]
struct SchematicNumber {
    value: i32,
    line: usize,
    span: Span,
}

impl SchematicNumber {
    fn new_with_values(value: i32, line: usize, span: Span) -> SchematicNumber {
        SchematicNumber { value, line, span }
    }
}

#[derive(Clone, PartialEq, Debug)]
struct Span {
    start: usize,
    end: usize,
}

impl Span {
    fn new() -> Span {
        Span { start: 0, end: 0 }
    }

    fn set_start(&mut self, start: usize) {
        self.start = start
    }

    fn set_end(&mut self, end: usize) {
        self.end = end
    }
}

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {}", err)
    }
}

fn run() -> Result<(), Day3Error> {
    let input_data = load_input("src/input.txt".to_string())?;
    let (schematic_numbers, schematic_symbols) = scan_schematic(input_data.to_owned())?;
    let sum = determine_gear_ratio(schematic_numbers, schematic_symbols);
    println!("sum of part numbers: {}", sum);
    Ok(())
}

fn load_input(file_path: String) -> Result<String, Day3Error> {
    let data = fs::read_to_string(file_path).map_err(Day3Error::UnableToLoadFile)?;
    println!("Successfully loaded file");
    Ok(data)
}

fn determine_gear_ratio(
    schematic_numbers: Vec<SchematicNumber>,
    schematic_symbols: Vec<SchematicSymbol>,
) -> i32 {
    let mut running_total = 0;

    for symbol in schematic_symbols {
        if let Some(gear_ratio) = find_gear_ratio(&schematic_numbers, symbol) {
            running_total += gear_ratio;
        }
    }

    running_total
}

fn find_gear_ratio(
    schematic_numbers: &Vec<SchematicNumber>,
    schematic_symbol: SchematicSymbol,
) -> Option<i32> {
    let mut found_numbers: Vec<i32> = Vec::new();

    for number in schematic_numbers {
        let positive_previous_line_number = match schematic_symbol.line {
            0 => 0,
            number => number - 1,
        };

        let positive_previous_index = match schematic_symbol.index {
            0 => 0,
            number => number - 1,
        };

        if number
            .line
            .within(positive_previous_line_number, schematic_symbol.line + 1)
            && (number
                .span
                .start
                .within(positive_previous_index, schematic_symbol.index + 1)
                || number
                    .span
                    .end
                    .within(positive_previous_index, schematic_symbol.index + 1))
        {
            found_numbers.push(number.value);
        }
    }

    if found_numbers.len() == 2 {
        Some(found_numbers.iter().product())
    } else {
        None
    }
}

fn scan_schematic(
    full_schematic: String,
) -> Result<(Vec<SchematicNumber>, Vec<SchematicSymbol>), Day3Error> {
    let mut schematic_numbers: Vec<SchematicNumber> = Vec::new();
    let mut schematic_symbols: Vec<SchematicSymbol> = Vec::new();

    for (line_number, line) in full_schematic.lines().enumerate() {
        scan_for_numbers_in_line(line.to_owned(), line_number)
            .map(|mut found_numbers| schematic_numbers.append(&mut found_numbers))?;
        schematic_symbols.append(&mut scan_for_symbols_in_line(line.to_owned(), line_number));
    }

    Ok((schematic_numbers, schematic_symbols))
}

fn scan_for_symbols_in_line(schematic_line: String, line_number: usize) -> Vec<SchematicSymbol> {
    let mut schematic_symbols: Vec<SchematicSymbol> = Vec::new();

    let chars_in_line = schematic_line.chars();
    for (i, char) in chars_in_line.clone().enumerate() {
        if char == '*' {
            schematic_symbols.push(SchematicSymbol {
                line: line_number,
                index: i,
            })
        }
    }

    schematic_symbols
}

fn scan_for_numbers_in_line(
    schematic_line: String,
    line_number: usize,
) -> Result<Vec<SchematicNumber>, Day3Error> {
    let mut schematic_numbers: Vec<SchematicNumber> = Vec::new();

    let mut constructed_number = String::new();
    let mut span = Span::new();

    let chars_in_line = schematic_line.chars();
    for (i, char) in chars_in_line.clone().enumerate() {
        if char.is_ascii_digit() {
            if constructed_number.is_empty() {
                span.set_start(i);
            }
            constructed_number.push(char);
            span.set_end(i);

            if i == chars_in_line.clone().count() - 1 && !constructed_number.is_empty() {
                construct_schematic_number(
                    &mut schematic_numbers,
                    &mut constructed_number,
                    line_number,
                    &span,
                )?;
            }
        } else if !constructed_number.is_empty() {
            construct_schematic_number(
                &mut schematic_numbers,
                &mut constructed_number,
                line_number,
                &span,
            )?;
        }
    }

    Ok(schematic_numbers)
}

fn construct_schematic_number(
    schematic_numbers: &mut Vec<SchematicNumber>,
    constructed_number: &mut String,
    line_number: usize,
    span: &Span,
) -> Result<(), Day3Error> {
    schematic_numbers.push(SchematicNumber::new_with_values(
        constructed_number.parse::<i32>()?,
        line_number,
        span.clone(),
    ));
    *constructed_number = String::new();
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{
        determine_gear_ratio, find_gear_ratio, load_input, scan_for_numbers_in_line,
        scan_for_symbols_in_line, scan_schematic, SchematicNumber, SchematicSymbol, Span,
    };

    #[test]
    fn find_number_in_line() {
        let input = "467.......";
        let result = scan_for_numbers_in_line(input.to_owned(), 1).unwrap();
        assert_eq!(
            result,
            vec![SchematicNumber::new_with_values(
                467,
                1,
                Span { start: 0, end: 2 }
            )]
        )
    }

    #[test]
    fn find_numbers_in_line() {
        let input = "467.*114.1";
        let result = scan_for_numbers_in_line(input.to_owned(), 1).unwrap();
        assert_eq!(
            result,
            vec![
                SchematicNumber::new_with_values(467, 1, Span { start: 0, end: 2 }),
                SchematicNumber::new_with_values(114, 1, Span { start: 5, end: 7 }),
                SchematicNumber::new_with_values(1, 1, Span { start: 9, end: 9 })
            ]
        )
    }

    #[test]
    fn find_symbol_in_line() {
        let input = "467...*...";
        let result = scan_for_symbols_in_line(input.to_owned(), 1);
        assert_eq!(result, vec![SchematicSymbol { line: 1, index: 6 }])
    }

    #[test]
    fn find_symbols_in_line() {
        let input = "467.3..*.@";
        let result = scan_for_symbols_in_line(input.to_owned(), 1);
        assert_eq!(result, vec![SchematicSymbol { line: 1, index: 7 },])
    }

    #[test]
    fn find_numbers_and_symbols_in_line() {
        let input = "467...*.@1";
        let result = scan_schematic(input.to_owned()).unwrap();
        assert_eq!(
            result,
            (
                vec![
                    SchematicNumber::new_with_values(467, 0, Span { start: 0, end: 2 }),
                    SchematicNumber::new_with_values(1, 0, Span { start: 9, end: 9 })
                ],
                vec![SchematicSymbol { line: 0, index: 6 },]
            )
        )
    }

    #[test]
    fn find_numbers_and_symbols_in_lines() {
        let input = "467..114..\n...*......";
        let result = scan_schematic(input.to_owned()).unwrap();
        assert_eq!(
            result,
            (
                vec![
                    SchematicNumber::new_with_values(467, 0, Span { start: 0, end: 2 }),
                    SchematicNumber::new_with_values(114, 0, Span { start: 5, end: 7 })
                ],
                vec![SchematicSymbol { line: 1, index: 3 },]
            )
        )
    }

    #[test]
    fn find_individual_gear_ratio() {
        /*
        4 6 7 . .
        . . . * .
        . 3 . 4 .
        */

        let input_schematic_numbers = vec![
            SchematicNumber::new_with_values(467, 0, Span { start: 0, end: 2 }),
            SchematicNumber::new_with_values(3, 2, Span { start: 1, end: 1 }),
            SchematicNumber::new_with_values(4, 2, Span { start: 3, end: 3 }),
        ];

        let input_schematic_symbol = SchematicSymbol { line: 1, index: 3 };

        let result = find_gear_ratio(&input_schematic_numbers, input_schematic_symbol).unwrap();

        assert_eq!(result, (467 * 4))
    }

    #[test]
    fn find_gear_ratio_sum() {
        let input = load_input("src/example_1.txt".to_string()).unwrap();
        let (schematic_numbers, schematic_symbols) = scan_schematic(input.to_owned()).unwrap();
        let result = determine_gear_ratio(schematic_numbers, schematic_symbols);
        assert_eq!(result, 467835)
    }
}
