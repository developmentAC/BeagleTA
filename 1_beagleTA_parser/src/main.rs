use colored::*;
use csv::Writer;
use regex::Regex;
use std::env;
use std::fs::{self, create_dir_all};
use std::io::{self, Write};
use std::path::Path;
// use std::process::Command; //used to run python scripts after generation

mod file_ops; // file operations
mod py_drivers; // the python code
mod text_analysis; // text analysis methods
mod toml_extract; // extract and print the version information according to the toml file

// Function to display a banner
fn show_banner() {
    // banner ref: https://manytools.org/hacker-tools/ascii-banner/
    // logo design: "ticks", use "█" to replace "/\" chars, "_" replaced with space
    let banner = String::from("
    _█ █ █ █ █________________________________________ █ █________________ █ █ █ █ █ █______ █ █_____
    _█ █____ █ █____ █ █ █____ █ █ █________ █ █ █ █__ █ █______ █ █ █________ █ █________ █ █ █ █___
    _█ █ █ █ █____ █ █ █ █ █______ █ █____ █ █__ █ █__ █ █____ █ █ █ █ █______ █ █______ █ █____ █ █_
    _█ █____ █ █__ █ █________ █ █ █ █______ █ █ █ █__ █ █____ █ █____________ █ █______ █ █ █ █ █ █_
    _█ █ █ █ █______ █ █ █ █__ █ █ █ █ █________ █ █__ █ █ █____ █ █ █ █______ █ █______ █ █____ █ █_
    _______________________________________ █ █ █ █___________________________________________________
    ");

    colour_print(&banner, "purple")
}

// Function to print text in different colors
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

// Main function
fn main() -> io::Result<()> {
    let output_dir = "0_out";
    create_dir_all(output_dir)?;

    show_banner();

    toml_extract::main(); // display version information.

    let intro_statement = String::from("\t Usage: cargo run <keyword_file.txt> <corpus_dir>");
    colour_print(&intro_statement, "yellow");

    let args: Vec<String> = env::args().collect();
    let (keyword_file_path, data_dir_path) = if args.len() >= 3 {
        (args[1].clone(), args[2].clone())
    } else {
        let keyword_file_path = get_input("\t 🐕  Enter the path to the keyword text file: ")?;
        let data_dir_path = get_input("\t 🐕  Enter the path to the data directory: ")?;
        (keyword_file_path, data_dir_path)
    };

    if keyword_file_path.is_empty() {
        colour_print("\t Keyword path cannot be empty!", "purple");
    }

    if data_dir_path.is_empty() {
        colour_print("\t Directory path cannot be empty!", "purple");
        return Ok(());
    }

    // Get the count of files in the data directory
    let file_count = file_ops::get_file_count(&data_dir_path);
    let out_message_0 = format!(
        "\t Found {} files in the directory '{}'.",
        file_count, &data_dir_path
    );
    colour_print(&out_message_0, "cyan");

    // Load stop words
    let stop_words = text_analysis::load_stop_words();

    // Read the keyword file content
    let keywords_content = fs::read_to_string(&keyword_file_path)?;

    // Tokenize the keywords without stemming
    let keywords: Vec<String> = text_analysis::tokenize(&keywords_content);

    let my_keywords: String = format!("\t Key words : {:?}", keywords);
    colour_print(&my_keywords, "yellow");

    // Create regex patterns from the keywords
    let regex_patterns: Vec<Regex> = keywords
        .iter()
        .filter_map(|k| Regex::new(&regex::escape(k)).ok())
        .collect();

    // Initialize CSV writer
    let mut writer = Writer::from_path(format!("{}/output.csv", output_dir))?; //copy for archival

    let headers = vec!["Filename".to_string()]
        .into_iter()
        .chain(keywords.clone())
        .chain(vec!["Cosine Similarity".to_string()]);
    writer.write_record(headers)?;

    let mut heatmap_data: Vec<Vec<f64>> = Vec::new();
    let mut file_names: Vec<String> = Vec::new();
    let mut cosine_similarities: Vec<f64> = Vec::new();

    // Process text files recursively
    file_ops::process_txt_files_recursively(
        Path::new(&data_dir_path),
        file_count,
        &regex_patterns,
        &stop_words,
        &mut heatmap_data,
        &mut file_names,
        &mut cosine_similarities,
        &mut writer,
    )?;

    // Write statistics to markdown
    text_analysis::write_statistics_to_markdown(&keywords, &heatmap_data, output_dir)?;

    // Generate and run Python script for visualization
    let python_script_path = format!("{}/visualizations.py", output_dir);
    fs::write(
        &python_script_path,
        py_drivers::generate_python_script(
            &keywords,
            &heatmap_data,
            &file_names,
            &cosine_similarities,
            output_dir,
        ),
    )?;

    // uhhh, this code does not always work, so I will just print the command to run it manually
    // colour_print("\t ✨ Running Python script for visualizations ...", "cyan");
    // Command::new("python3").arg(&python_script_path).status()?;

    // Generate and run Python script for network visualization
    let python_script_path = format!("{}/interactive_network_1.py", output_dir);
    fs::write(
        &python_script_path,
        py_drivers::generate_python_script_network_1(output_dir),
    )?;

    colour_print(
        &format!("\n\t ✨ Run commands to create virtual environment with Python for the following commands ..."),
        "yellow",
    );

    colour_print(
        &format!(
            "\t python3 -m venv ~/Desktop/venv
\t source ~/Desktop/venv/bin/activate
\t pip install plotly matplotlib numpy pandas networkx seaborn scikit-learn pyvis
"
        ),
        "cyan",
    );

    colour_print(
        &format!("\t ✨ Run commands to create keyword analysis and other visualizations ..."),
        "yellow",
    );

    colour_print(
        &format!("\t cd 0_out && python3 visualizations.py && cd .."),
        "cyan",
    );

    colour_print(
        &format!("\t cd 0_out && python3 interactive_network_1.py --datafile output.csv --threshold 6 && cd .. "),
        "cyan",
    );

    let python_script_path = format!("{}/interactive_network_2.py", output_dir);
    fs::write(
        &python_script_path,
        py_drivers::generate_python_script_network_2(output_dir),
    )?;

    colour_print(
        &format!("\t cd 0_out && python3 interactive_network_2.py --datafile output.csv && cd .. "),
        "cyan",
    );

    let python_script_path = format!("{}/complete_network.py", output_dir);
    fs::write(
        &python_script_path,
        py_drivers::generate_python_script_complete_network(output_dir),
    )?;

    colour_print(
        &format!("\t cd 0_out && python3 complete_network.py -f output.csv -t 6 -nc blue -ec green  && cd .. \n"),
        "cyan",
    );

    Ok(())
} //end of main()

// Function to get user input from the terminal
fn get_input(prompt: &str) -> io::Result<String> {
    let mut input = String::new();
    print!("{}", prompt);
    io::stdout().flush()?;
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
} // end of get_input()
