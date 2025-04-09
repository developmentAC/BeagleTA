use clap::{Arg, Command};
use csv::{ReaderBuilder, WriterBuilder};
use rand::{seq::SliceRandom, thread_rng};
use std::error::Error;
use std::fs::File;
// use std::path::Path;

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
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::new("sample_size")
                .long("sample_size")
                .short('s')
                .value_name("SIZE")
                .help("Number of rows to randomly sample")
                .takes_value(true)
                .default_value("10"),
        )
        .arg(
            Arg::new("output")
                .long("output")
                .short('o')
                .value_name("FILE")
                .help("Prefix for output CSV files")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::new("num_files")
                .long("num_files")
                .short('n')
                .value_name("NUM")
                .help("Number of output files to create")
                .takes_value(true)
                .default_value("1"),
        )
        .get_matches();

    // Extract arguments
    let input_file = matches.value_of("input").unwrap();
    let sample_size: usize = matches.value_of_t("sample_size")?;
    let output_prefix = matches.value_of("output").unwrap();
    let num_files: usize = matches.value_of_t("num_files")?;

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
        let output_file = File::create(output_filename)?;

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

// fn main() {
//     println!("Hello, world!");
// }
