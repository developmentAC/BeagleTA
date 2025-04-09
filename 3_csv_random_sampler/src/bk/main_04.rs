use clap::{Arg, Command};
use csv::{ReaderBuilder, WriterBuilder};
use rand::{seq::SliceRandom, thread_rng};
use std::error::Error;
use std::fs::{File, create_dir_all};
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    // Parse the command line arguments
    let matches = Command::new("csv_random_sampler")
        .version("1.0")
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
    let mut rdr = ReaderBuilder::new().has_headers(true).from_path(input_file)?;
    let headers = rdr.headers()?.clone();
    let records: Vec<_> = rdr.records().collect::<Result<_, _>>()?;

    // Perform the random sampling and save the output
    let mut rng = thread_rng();

    for i in 0..num_files {
        let sampled_data: Vec<_> = records.choose_multiple(&mut rng, sample_size).cloned().collect();

        // Create the output filename inside the "0_out/" directory
        let output_filename = format!(
            "{}_{:02}.csv",
            output_prefix, 
            i + 1
        );
        let output_path = Path::new(output_dir).join(&output_filename);

        // Write the sampled data to the output CSV file
        let output_file = File::create(output_path)?;
        let mut wtr = WriterBuilder::new().has_headers(true).from_writer(output_file);
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