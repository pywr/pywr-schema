use clap::Parser;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufReader;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    path: std::path::PathBuf,
}

fn main() {
    let args = Args::parse();

    println!("Path: {:?}", args.path);

    let file = File::open(args.path).expect("Could not open file.");
    let reader = BufReader::new(file);

    let model: pywr_schema::PywrModel =
        serde_json::from_reader(reader).expect("Failed to parse Pywr JSON file.");

    println!("Parsed Pywr JSON file successfully!");

    {
        // Identify custom nodes
        let custom_types: HashSet<_> = model
            .nodes
            .iter()
            .filter_map(|n| {
                if let pywr_schema::nodes::Node::Custom(c) = n {
                    println!("Custom node: {:#?}", c);
                    Some(c.ty.clone())
                } else {
                    None
                }
            })
            .collect();

        let mut custom_types: Vec<_> = custom_types.into_iter().collect();
        custom_types.sort();

        if !custom_types.is_empty() {
            println!("Found {} custom node types:", custom_types.len());
            for ty in custom_types {
                println!("  {}", ty);
            }
        } else {
            println!("No custom nodes found!")
        }
    }

    if let Some(parameters) = model.parameters {
        let custom_types: HashSet<_> = parameters
            .iter()
            .filter_map(|p| {
                if let pywr_schema::parameters::Parameter::Custom(c) = p {
                    Some(c.ty.clone())
                } else {
                    None
                }
            })
            .collect();

        let mut custom_types: Vec<_> = custom_types.into_iter().collect();
        custom_types.sort();

        if !custom_types.is_empty() {
            println!("Found {} custom parameter types:", custom_types.len());
            for ty in custom_types {
                println!("  {}", ty);
            }
        } else {
            println!("No custom parameters found!")
        }
    }
}
