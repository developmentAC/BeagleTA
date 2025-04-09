use clap::{Arg, Command};
use colored::*;
use csv::{ReaderBuilder, WriterBuilder};
use std::io;
use std::{error::Error, fs::File, io::Write};

fn show_banner() {
    // banner ref: https://manytools.org/hacker-tools/ascii-banner/

    //logo design: "ticks", use "█" to replace "/\" chars, "_" replaced with space
    let banner = String::from(
        "

    ___█ █ █ █ █ __█ █ _______________________________________________________________
    _█ █ __________█ █ ______█ █ █ ____█ █ █ ______█ █ █ █ ______█ █ █ ____█ █ __█ █ _
    _█ █ __________█ █ ____█ █ █ █ █ ______█ █ ____█ █ __█ █ __█ █ █ █ █ __█ █ █ █ ___
    _█ █ __________█ █ ____█ █ ________█ █ █ █ ____█ █ __█ █ __█ █ ________█ █ _______
    ___█ █ █ █ █ __█ █ █ ____█ █ █ █ __█ █ █ █ █ __█ █ __█ █ ____█ █ █ █ __█ █ _______
    __________________________________________________________________________________
    
        ",
    );

    colour_print(&banner, "purple")
}

fn colour_print(text: &str, colour: &str) {
    match colour {
        "flush_green" => {
            print!("\x1b[2K\r"); // Clear the line and move to the beginning
            io::stdout().flush().unwrap();
            print!(" {}", text.bright_green().bold());
            io::stdout().flush().unwrap();
        }
        "green" => {
            print!("\x1b[2K\r"); // Clear the line and move to the beginning
            println!("{}", text.bright_green().bold());
        }
        "red" => {
            print!("\x1b[2K\r"); // Clear the line and move to the beginning
            println!("{}", text.bright_red().bold());
        }
        "cyan" => {
            print!("\x1b[2K\r"); // Clear the line and move to the beginning
            println!("{}", text.bright_cyan().bold());
        }
        "purple" => {
            print!("\x1b[2K\r"); // Clear the line and move to the beginning
            println!("{}", text.bright_purple().bold());
        }
        "blue" => {
            print!("\x1b[2K\r"); // Clear the line and move to the beginning
            println!("{}", text.bright_blue().bold());
        }
        "yellow" => {
            print!("\x1b[2K\r"); // Clear the line and move to the beginning
            println!("{}", text.bright_yellow().bold());
        }
        _ => {
            print!("\x1b[2K\r"); // Clear the line and move to the beginning
            println!("{}", text.bright_yellow().bold());
        }
    }
} // end of colour_print()

fn main() -> Result<(), Box<dyn Error>> {
    // show the banner
    show_banner();

    // Set up command-line argument parsing with clap v4.x

    let matches = Command::new("CSV Filter")
        .version("1.0")
        .author("Oliver Bonham-Carter <obonhamcarter@allegheny.edu>")
        .about("Removes rows from a CSV where a specified column contains zero.")
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .value_parser(clap::value_parser!(String))
                .required(true)
                .help("Input CSV file"),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_parser(clap::value_parser!(String))
                .required(true)
                .help("Output CSV file"),
        )
        .arg(
            Arg::new("column")
                .short('c')
                .long("column")
                .value_parser(clap::value_parser!(usize))
                .required(true)
                .help("Column index (1-based) to check for non-zero value"),
        )
        .arg(
            Arg::new("remove-column")
                .short('r')
                .long("remove-column")
                .value_parser(clap::value_parser!(usize))
                .help("Column index (1-based) to remove from the dataset"),
        )
        .get_matches();

    // Get the input, output, and column arguments
    let input_file = matches.get_one::<String>("input").unwrap();
    let output_file = matches.get_one::<String>("output").unwrap();
    let column_index: usize = *matches.get_one::<usize>("column").unwrap();
    let remove_column_index: Option<usize> = matches.get_one::<usize>("remove-column").copied();

    // Open the input CSV file
    let input = File::open(input_file)?;
    let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(input);

    // Create a writer for the output CSV file
    let output = File::create(output_file)?;
    let mut wtr = WriterBuilder::new().from_writer(output);

    // Read the headers
    let headers = rdr.headers()?.clone();

    // If there is a column to remove, adjust the headers
    let adjusted_headers: Vec<String> = match remove_column_index {
        Some(index) => headers
            .iter()
            .enumerate()
            .filter_map(|(i, field)| {
                if i == index - 1 {
                    None
                } else {
                    Some(field.to_string())
                }
            })
            .collect(),
        None => headers.iter().map(|s| s.to_string()).collect(),
    };

    // Write the adjusted headers to the output file
    wtr.write_record(&adjusted_headers)?;

    // Create a report file to record removed rows
    let mut report = File::create("removed_rows.txt")?;

    let mut removed_count = 0;

    // Process the rows in the CSV
    for result in rdr.records() {
        let record = result?;
        let filename = record.get(0).unwrap().to_string();
        let data: Vec<f64> = record
            .iter()
            .skip(1)
            .map(|v| v.parse().unwrap_or(0.0))
            .collect();

        // Check if the value in the user-specified column is non-zero
        if let Some(column_value) = record.get(column_index - 1) {
            if column_value.parse::<f64>().unwrap_or(0.0) != 0.0 {
                // If the column value is non-zero, write the row, removing the specified column if necessary
                let adjusted_record: Vec<String> = record
                    .iter()
                    .enumerate()
                    .filter_map(|(i, field)| {
                        // If the current index matches the remove column, skip it
                        if Some(i + 1) == remove_column_index {
                            None
                        } else {
                            Some(field.to_string())
                        }
                    })
                    .collect();

                // Write the adjusted record to the output
                wtr.write_record(&adjusted_record)?;
            } else {
                // Otherwise, record it in the report file
                writeln!(report, "{} was removed: {:?}", filename, data)?;
                removed_count += 1;
            }
        }
    }

    println!("Rows removed: {}", removed_count);

    // Ensure everything is written out
    wtr.flush()?;
    report.flush()?;

    Ok(())
}

// Options:
//   -i, --input <input>                  Input CSV file
//   -o, --output <output>                Output CSV file
//   -c, --column <column>                Column index (1-based) to check for non-zero value
//   -r, --remove-column <remove-column>  Column index (1-based) to remove from the dataset
//   -h, --help                           Print help
//   -V, --version                        Print version

// run : cargo run -- --input input.csv --output output.csv --column 2 --remove-column 3
// Check for non-zero values in the second column (--column 2).
// Remove the third column (--remove-column 3) from both the header and the dataset.

// run:
// cargo run -- --input input.csv --output output.csv --column 2 --remove-column 3 && head input.csv output.csv

// Example: for data,
// ==> input.csv <==
// Filename,2024,kinase,ethic,Cosine Similarity
// PMC514601.txt,0,0,0,0.0000
// PMC516016.txt,1,1,0,0.8165
// PMC549512.txt,0,0,0,0.0000
// PMC516770.txt,0,0,0,0.0000
// PMC514615.txt,0,0,0,0.0000
// PMC535803.txt,0,0,0,0.0000
// PMC521686.txt,0,0,0,0.0000
// PMC314466.txt,0,7,0,0.5774
// PMC314300.txt,0,5,0,0.5774

// ==> output.csv <==
// Filename,2024,kinase,ethic,Cosine Similarity
// PMC516016.txt,1,1,0,0.8165
// PMC552330.txt,1,1,0,0.8165
// PMC533862.txt,1,55,0,0.5878
// PMC193605.txt,1,1,0,0.8165
// PMC545077.txt,1,0,0,0.5774
// PMC546191.txt,1,0,2,0.7746
// PMC548143.txt,2,30,0,0.6145
// PMC535816.txt,1,0,1,0.8165
// PMC546408.txt,1,0,0,0.5774
//
// Column 2 (or the second along from the left) must be non-zero.
//  cargo run -- --input input.csv --output output.csv --column 2 && head input.csv output.csv
