use std::error::Error;
use std::fs::{copy, read_to_string, write};
use std::path::PathBuf;
use std::str::FromStr;

use clap::Parser;
use toml::Table;

#[derive(clap::Parser, Debug)]
struct Arguments {
    number: u16,

    #[arg(short = 'd', long)]
    model_dir: Option<PathBuf>,

    #[arg(short = 'o', long)]
    output_dir: Option<PathBuf>,

    #[arg(short = 'f', long, default_value_t = String::from("file"))]
    model_file_name: String,

    #[arg(short = 'e', long)]
    model_file_extensions: Option<Vec<String>>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let Arguments {
        number,
        model_file_extensions,
        model_file_name,
        model_dir: model_path,
        output_dir: output_path,
    } = Arguments::parse();

    let model_path = model_path.unwrap_or("./model".into());
    let model_file_extensions =
        model_file_extensions.unwrap_or(vec!["rs".to_string(), "alg".to_string()]);
    let output_path = output_path.unwrap_or("./src".into());

    let files_path = model_file_extensions.iter().map(|extension| {
        (
            model_path.join(format!("{model_file_name}.{extension}")),
            extension,
        )
    });

    let final_number = format!("{number:03}");
    let mut rust_path = None;

    for (file, extension) in files_path {
        if extension == "rs" {
            rust_path = Some(output_path.join(format!("{final_number}.{extension}")))
        }
        copy(
            &file,
            output_path.join(format!("{final_number}.{extension}")),
        )?;
    }

    if model_file_extensions.contains(&"rs".to_string()) {
        let cargo_toml = read_to_string("./Cargo.toml")?;
        let mut cargo_table: Table = cargo_toml.parse().unwrap();

        cargo_table["bin"]
            .as_array_mut()
            .unwrap()
            .push(toml::Value::Table(
                Table::from_str(&format!(
                    "name = \"{final_number}\"\npath = \"{}\"",
                    rust_path.unwrap().to_str().unwrap()
                ))
                .unwrap(),
            ));

        write("./Cargo.toml", format!("{}", cargo_table).as_bytes()).unwrap();
    }

    Ok(())
}
