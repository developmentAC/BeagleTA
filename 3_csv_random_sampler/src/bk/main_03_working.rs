use clap::{Arg, Command};
use csv::{ReaderBuilder, WriterBuilder};
use rand::{seq::SliceRandom, thread_rng};
use std::error::Error;
use std::fs::File;

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
                // .default_value_else(|| 10),  // Default value for sample_size
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
                // .default_value_else(|| 1),  // Default value for num_files
        )
        .get_matches();

    // Extract arguments
    let input_file = matches.get_one::<String>("input").unwrap();
    let sample_size: usize = *matches.get_one::<usize>("sample_size").unwrap();
    let output_prefix = matches.get_one::<String>("output").unwrap();
    let num_files: usize = *matches.get_one::<usize>("num_files").unwrap();

    // Load the CSV data from the input file
    let mut rdr = ReaderBuilder::new().has_headers(true).from_path(input_file)?;
    let headers = rdr.headers()?.clone();
    let records: Vec<_> = rdr.records().collect::<Result<_, _>>()?;

    // Perform the random sampling and save the output
    let mut rng = thread_rng();

    for i in 0..num_files {
        let sampled_data: Vec<_> = records.choose_multiple(&mut rng, sample_size).cloned().collect();

        // Create the output filename
        let output_filename = format!(
            "{}_{:02}.csv",
            output_prefix, 
            i + 1
        );
        let output_file = File::create(&output_filename)?;

        // Write the sampled data to the output CSV file
        let mut wtr = WriterBuilder::new().has_headers(true).from_writer(output_file);
        wtr.write_record(&headers)?;
        for record in sampled_data {
            wtr.write_record(record.iter())?;
        }
        wtr.flush()?;
        println!("Saved random sample to {}", output_filename);
    }

    Ok(())
}


// run: cargo run -- --input data.csv --sample_size 10 --output output --num_files 1
