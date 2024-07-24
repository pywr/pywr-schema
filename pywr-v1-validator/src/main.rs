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
    #[arg(short, long)]
    network_only: bool,
}

fn main() {
    let args = Args::parse();

    println!("Path: {:?}", args.path);

    let file = File::open(args.path).expect("Could not open file.");
    let reader = BufReader::new(file);

    let network: pywr_v1_schema::PywrNetwork = if args.network_only {
        serde_json::from_reader(reader).expect("Failed to parse Pywr JSON file.")
    } else {
        let model: pywr_v1_schema::PywrModel =
            serde_json::from_reader(reader).expect("Failed to parse Pywr JSON file.");

        model.network
    };

    println!("Parsed Pywr JSON file successfully!");

    {
        // Identify custom nodes
        let custom_types: HashSet<_> = match network.nodes {
            Some(nodes) => nodes
                .iter()
                .filter_map(|n| {
                    if let pywr_v1_schema::nodes::Node::Custom(c) = n {
                        Some(c.ty.clone())
                    } else {
                        None
                    }
                })
                .collect(),
            None => HashSet::new(),
        };

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

    if let Some(parameters) = network.parameters {
        let custom_types: HashSet<_> = parameters
            .iter()
            .filter_map(|p| {
                if let pywr_v1_schema::parameters::Parameter::Custom(c) = p {
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
