use crate::colour_print;
use crate::fs;
use crate::io;
use crate::text_analysis;
use crate::text_analysis::remove_stop_words;
use csv::Writer;
use jwalk::WalkDir;
use regex::Regex;
use std::collections::HashSet;
use std::path::Path;

// Function to count the number of files in a directory
pub fn get_file_count(dir: &str) -> i32 {
    let mut count = 0;
    for entry in WalkDir::new(dir).skip_hidden(false) {
        if let Ok(_) = entry {
            count += 1;
        }
    }
    count
}

// Function to load each file in the data directory and complete text processing
pub fn process_txt_files_recursively(
    dir: &Path,
    file_count: i32,
    regex_patterns: &[Regex],
    stop_words: &HashSet<String>,
    heatmap_data: &mut Vec<Vec<f64>>,
    file_names: &mut Vec<String>,
    cosine_similarities: &mut Vec<f64>,
    writer: &mut Writer<std::fs::File>,
) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        // Ignore hidden files or directories (those starting with '.')
        if let Some(file_name) = path.file_name() {
            if file_name.to_string_lossy().starts_with('.') {
                continue;
            }
        }

        if path.is_dir() {
            // Recursively process subdirectories
            process_txt_files_recursively(
                &path,
                file_count,
                regex_patterns,
                stop_words,
                heatmap_data,
                file_names,
                cosine_similarities,
                writer,
            )?;
        } else if path.is_file() {
            // Only process .txt files
            if let Some(extension) = path.extension() {
                if extension == "txt" {
                    let file_name = path.file_name().unwrap().to_string_lossy().to_string();

                    // Let the user know which files are being processed
                    let update_message = format!(
                        "\t * {} -> {} of {}",
                        file_name,
                        file_names.len() + 1,
                        file_count
                    );
                    colour_print(&update_message, "flush_green");

                    if let Ok(file_content) = fs::read_to_string(&path) {
                        // Remove stop words from the file content
                        let cleaned_content = remove_stop_words(&file_content, stop_words);

                        // Count occurrences of each regex pattern in the cleaned content
                        let counts: Vec<usize> = regex_patterns
                            .iter()
                            .map(|regex| regex.find_iter(&cleaned_content).count())
                            .collect();

                        // Calculate cosine similarity based on the counts
                        let cosine_similarity =
                            text_analysis::calculate_cosine_similarity(&counts)?;

                        // Prepare a row for CSV output
                        let mut row: Vec<String> = vec![file_name.clone()];
                        row.extend(counts.iter().map(|count| count.to_string()));
                        row.push(format!("{:.4}", cosine_similarity));
                        writer.write_record(&row)?;

                        // Update heatmap data and cosine similarities
                        heatmap_data.push(
                            counts
                                .iter()
                                .map(|&x| if x > 0 { (x as f64).ln() } else { 0.0 })
                                .collect::<Vec<f64>>(),
                        );
                        cosine_similarities.push(cosine_similarity);
                        file_names.push(file_name);
                    }
                }
            }
        }
    }
    Ok(())
} // end of process_txt_files_recursively()
