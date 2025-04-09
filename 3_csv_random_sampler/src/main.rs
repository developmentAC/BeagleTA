use clap::{Arg, Command};
use colored::*;
use csv::{ReaderBuilder, WriterBuilder};
use rand::{seq::SliceRandom, thread_rng};
use std::error::Error;
use std::fs::{create_dir_all, File};
use std::io;
use std::io::Write;
use std::path::Path;

fn show_banner() {
    // banner ref: https://manytools.org/hacker-tools/ascii-banner/
    //logo design: "ticks", use "█" to replace "/\" chars, "_" replaced with space
    let banner = String::from(
        "
 
_____________________________________________________█ █ ___________________________
___█ █ █ █ __█ █ █ ______█ █ █ __█ █ ____█ █ █ █ ____█ █ ______█ █ █ ____█ █ __█ █ _
_█ █ █ █ ________█ █ ____█ █ █ █ █ █ █ __█ █ __█ █ __█ █ ____█ █ █ █ █ __█ █ █ █ ___
_______█ █ __█ █ █ █ ____█ █ __█ __█ █ __█ █ █ █ ____█ █ ____█ █ ________█ █ _______
_█ █ █ █ ____█ █ █ █ █ __█ █ ______█ █ __█ █ ________█ █ █ ____█ █ █ █ __█ █ _______
_________________________________________█ █ _______________________________________

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

    // Parse the command line arguments
    let matches = Command::new("csv_random_sampler")
        .version("1.0")
        .author("Oliver Bonham-Carter <obonhamcarter@allegheny.edu>")
        .about("Randomly samples data from a CSV file")
        .arg(
            Arg::new("input")
                .long("input")
                .short('i')
                .value_name("FILE")
                .help("Input CSV file to sample from")
                .value_parser(clap::value_parser!(String))
                .required(true),
        )
        .arg(
            Arg::new("sample_size")
                .long("sample_size")
                .short('s')
                .value_name("SIZE")
                .help("Number of rows to randomly sample")
                .value_parser(clap::value_parser!(usize))
                .required(true),
        )
        .arg(
            Arg::new("output")
                .long("output")
                .short('o')
                .value_name("FILE")
                .help("Prefix for output CSV files")
                .value_parser(clap::value_parser!(String))
                .required(true),
        )
        .arg(
            Arg::new("num_files")
                .long("num_files")
                .short('n')
                .value_name("NUM")
                .help("Number of output files to create")
                .value_parser(clap::value_parser!(usize))
                .required(true),
        )
        .get_matches();

    // Extract arguments
    let input_file = matches.get_one::<String>("input").unwrap();
    let sample_size: usize = *matches.get_one::<usize>("sample_size").unwrap();
    let output_prefix = matches.get_one::<String>("output").unwrap();
    let num_files: usize = *matches.get_one::<usize>("num_files").unwrap();

    // Ensure the output directory exists
    let output_dir = "0_out/";
    create_dir_all(output_dir)?;

    // Load the CSV data from the input file
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_path(input_file)?;
    let headers = rdr.headers()?.clone();
    let records: Vec<_> = rdr.records().collect::<Result<_, _>>()?;

    // Perform the random sampling and save the output
    let mut rng = thread_rng();

    for i in 0..num_files {
        let sampled_data: Vec<_> = records
            .choose_multiple(&mut rng, sample_size)
            .cloned()
            .collect();

        // Create the output filename inside the "0_out/" directory
        let output_filename = format!("{}_{:02}.csv", output_prefix, i + 1);
        let output_path = Path::new(output_dir).join(&output_filename);

        // Write the sampled data to the output CSV file
        let output_file = File::create(output_path)?;
        let mut wtr = WriterBuilder::new()
            .has_headers(true)
            .from_writer(output_file);
        wtr.write_record(&headers)?;
        for record in sampled_data {
            wtr.write_record(record.iter())?;
        }
        wtr.flush()?;
        println!("Saved random sample to {}/{}", output_dir, output_filename);
    }

    Ok(())
}

// run: cargo run -- --input data.csv --sample_size 10 --output output --num_files 1
// run: cargo run -- --input data.csv --sample_size 10 --output output --num_files 1
