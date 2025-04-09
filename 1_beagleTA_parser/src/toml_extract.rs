use crate::colour_print;
use serde::Deserialize;
use std::fs;
use toml::de::from_str;

// Struct to represent the package information in Cargo.toml
#[derive(Debug, Deserialize)]
struct Package {
    name: String,
    version: String,
    edition: String,
}

// Struct to represent the entire Cargo.toml file
#[derive(Debug, Deserialize)]
struct CargoToml {
    package: Package,
}

// Function to parse the Cargo.toml file and extract package information
fn parse_cargo_toml(file_path: &str) {
    // Read the content of the Cargo.toml file
    let content = fs::read_to_string(file_path).expect("Failed to read Cargo.toml file");

    // Parse the TOML content into the CargoToml struct
    let cargo_toml: CargoToml = from_str(&content).expect("Failed to parse Cargo.toml");

    // Print the extracted package information
    let out_message_0 = format!("\t Package name: '{}'.", cargo_toml.package.name);
    colour_print(&out_message_0, "purple");

    let out_message_1 = format!("\t Package version: '{}'.", cargo_toml.package.version);
    colour_print(&out_message_1, "purple");

    let out_message_2 = format!("\t Package edition: '{}'.\n", cargo_toml.package.edition);
    colour_print(&out_message_2, "purple");
}

// Main function to be called from the main.rs
pub fn main() {
    let file_path = "Cargo.toml"; // Path to your Cargo.toml file
    parse_cargo_toml(file_path);
}
