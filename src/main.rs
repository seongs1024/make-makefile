use clap::{Parser, ValueEnum};
use std::io::Write;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the project
    #[arg(short, long)]
    name: Option<String>,

    /// Binary or library
    #[arg(short, long, value_enum, default_value_t = ProjectKind::Bin)]
    kind: ProjectKind,
}

#[derive(Clone, PartialEq, ValueEnum)]
enum ProjectKind {
    Bin,
    Lib,
}

fn make(name: Option<String>, kind: ProjectKind) -> Result<(), Box<dyn std::error::Error>> {
    let origin: String = include_str!("../Makefile.example").to_owned();

    let origin = if let Some(name) = name {
        let name = match kind {
            ProjectKind::Bin => name,
            ProjectKind::Lib => format!("lib{}.a", name),
        };
        origin.replacen("{PROGRAM}", &name, 1)
    } else {
        origin
    };
    let mut out_file = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open("Makefile")?;
    out_file.write_all(origin.as_bytes())?;
    Ok(())
}

fn main() {
    let args = Args::parse();
    match make(args.name, args.kind) {
        Ok(()) => println!("Makefile has been generated."),
        Err(e) => println!("{}", e),
    }
}
