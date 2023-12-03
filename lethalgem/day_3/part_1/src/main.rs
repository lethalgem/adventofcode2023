use cond_utils::Between;
use std::{fs, io, num::ParseIntError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Day3Error {
    #[error("File not loaded")]
    UnableToLoadFile(#[from] io::Error),
    #[error("Could not find number in line")]
    NoNumberFound,
    #[error("Failed to parse int from string")]
    ParseIntFailed(#[from] ParseIntError),
    #[error("Could not find colon in line")]
    NoColonFound,
}

/*
Todo:
1. split into lines
2. go through char by char
3. find each digit
4. on digit find, check adjacent for a symbol -- set flag if true
5. combine horizontally adjacent digits into number
4. keep a list of numbers
5. sum all numbers that have symbol
 */

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
    let part_numbers = determine_part_numbers(schematic_numbers, schematic_symbols)?;
    let sum = sum_part_numbers(part_numbers);
    println!("sum of part numbers: {}", sum);
    Ok(())
}

fn load_input(file_path: String) -> Result<String, Day3Error> {
    let data = fs::read_to_string(file_path).map_err(Day3Error::UnableToLoadFile)?;
    println!("Successfully loaded file");
    Ok(data)
}

fn sum_part_numbers(part_numbers: Vec<SchematicNumber>) -> i32 {
    part_numbers.iter().map(|part| part.value).sum()
}

fn determine_part_numbers(
    schematic_numbers: Vec<SchematicNumber>,
    schematic_symbols: Vec<SchematicSymbol>,
) -> Result<Vec<SchematicNumber>, Day3Error> {
    let mut part_numbers: Vec<SchematicNumber> = Vec::new();

    for number in schematic_numbers {
        if is_adjacent_to_symbol(&number, schematic_symbols.clone()) {
            part_numbers.push(number);
        }
    }

    Ok(part_numbers)
}

fn is_adjacent_to_symbol(
    schematic_number: &SchematicNumber,
    schematic_symbols: Vec<SchematicSymbol>,
) -> bool {
    for symbol in schematic_symbols {
        let positive_previous_line_number = match schematic_number.line {
            0 => 0,
            number => number - 1,
        };

        let positive_span_start = match schematic_number.span.start {
            0 => 0,
            number => number - 1,
        };

        if symbol
            .line
            .within(positive_previous_line_number, schematic_number.line + 1)
            && symbol
                .index
                .within(positive_span_start, schematic_number.span.end + 1)
        {
            return true;
        }
    }

    false
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
        if !char.is_ascii_digit() && char != '.' {
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
        determine_part_numbers, is_adjacent_to_symbol, load_input, scan_for_numbers_in_line,
        scan_for_symbols_in_line, scan_schematic, sum_part_numbers, SchematicNumber,
        SchematicSymbol, Span,
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
        assert_eq!(
            result,
            vec![
                SchematicSymbol { line: 1, index: 7 },
                SchematicSymbol { line: 1, index: 9 }
            ]
        )
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
                vec![
                    SchematicSymbol { line: 0, index: 6 },
                    SchematicSymbol { line: 0, index: 8 }
                ]
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
    fn find_part_number() {
        let mut results: Vec<bool> = Vec::new();

        /*
         * * * * *
         * 4 6 7 *
         * * * * *
         */

        let input_schematic_number =
            SchematicNumber::new_with_values(467, 1, Span { start: 1, end: 3 });
        let input_schematic_symbols = vec![
            SchematicSymbol { line: 0, index: 0 },
            SchematicSymbol { line: 0, index: 1 },
            SchematicSymbol { line: 0, index: 2 },
            SchematicSymbol { line: 0, index: 3 },
            SchematicSymbol { line: 0, index: 4 },
            SchematicSymbol { line: 1, index: 0 },
            SchematicSymbol { line: 1, index: 4 },
            SchematicSymbol { line: 2, index: 0 },
            SchematicSymbol { line: 2, index: 1 },
            SchematicSymbol { line: 2, index: 2 },
            SchematicSymbol { line: 2, index: 3 },
            SchematicSymbol { line: 2, index: 4 },
        ];

        for symbol in input_schematic_symbols {
            results.push(is_adjacent_to_symbol(&input_schematic_number, vec![symbol]))
        }
        assert_eq!(
            results,
            vec![true, true, true, true, true, true, true, true, true, true, true, true,]
        )
    }

    #[test]
    fn throw_out_schematic_number() {
        let mut results: Vec<bool> = Vec::new();

        /*
        . . . . .
        . 4 6 7 .
        . . . . .
        */

        let input_schematic_number =
            SchematicNumber::new_with_values(467, 1, Span { start: 1, end: 3 });
        let input_schematic_symbols = vec![];

        for symbol in input_schematic_symbols {
            results.push(is_adjacent_to_symbol(&input_schematic_number, vec![symbol]))
        }
        assert_eq!(results, vec![])
    }

    #[test]
    fn find_part_numbers_in_line() {
        let input = "467..114*..\n...*......";
        let (schematic_numbers, schematic_symbols) = scan_schematic(input.to_owned()).unwrap();
        let result = determine_part_numbers(schematic_numbers, schematic_symbols).unwrap();
        assert_eq!(
            result,
            vec![
                SchematicNumber {
                    value: 467,
                    line: 0,
                    span: Span { start: 0, end: 2 }
                },
                SchematicNumber {
                    value: 114,
                    line: 0,
                    span: Span { start: 5, end: 7 }
                }
            ]
        )
    }

    #[test]
    fn sum_part_numbers_in_full_schematic() {
        let input = load_input("src/example_1.txt".to_string()).unwrap();
        let (schematic_numbers, schematic_symbols) = scan_schematic(input.to_owned()).unwrap();
        let part_numbers = determine_part_numbers(schematic_numbers, schematic_symbols).unwrap();
        let result = sum_part_numbers(part_numbers);
        assert_eq!(result, 4361)
    }
}
