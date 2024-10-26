use std::{collections::BTreeMap, fs::File, io::Write, path::PathBuf};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long)]
    from: PathBuf,

    #[arg(long)]
    into: PathBuf,
}

#[derive(Debug, Clone)]
pub struct Hole {
    pub section: Option<String>,
    pub line: usize,
    pub key: String,
    pub raw_value: String,
    pub raw_line: String,
}

#[derive(Debug, Clone, Ord, Eq, PartialEq, PartialOrd)]
pub struct Path {
    pub section: Option<String>,
    pub key: String,
}

pub fn parse(contents: &str) -> BTreeMap<Path, Hole> {
    let mut holes: BTreeMap<Path, Hole> = BTreeMap::new();

    let mut section: Option<String> = None;

    for (i, line) in contents.lines().enumerate() {
        let trimmed = line.trim();

        if trimmed.starts_with('[') {
            eprintln!("Section: {}", trimmed);
            section = Some(trimmed.to_string());
        }

        if trimmed.contains('=') && trimmed.contains("{{") && trimmed.contains("}}") {
            let key = trimmed.split('=').next().unwrap().trim();
            let value = trimmed.split('=').nth(1).unwrap().trim();

            let hole = Hole {
                section: section.clone(),
                line: i,
                key: key.to_string(),
                raw_value: value.to_string(),
                raw_line: line.to_string(),
            };

            let path = Path {
                section: section.clone(),
                key: key.to_string(),
            };

            holes.insert(path, hole.clone());

            eprintln!("Hole: {:?}", hole);
        }
    }

    holes
}

pub fn apply(contents: &str, holes: BTreeMap<Path, Hole>) -> String {
    let mut buf = String::new();
    let mut section: Option<String> = None;

    for line in contents.lines() {
        let trimmed = line.trim();

        if trimmed.starts_with('[') {
            eprintln!("Section: {}", trimmed);
            section = Some(trimmed.to_string());
        }

        if trimmed.contains('=') {
            let key = trimmed.split('=').next().unwrap().trim();

            let path = Path {
                section: section.clone(),
                key: key.to_string(),
            };

            match holes.get(&path) {
                Some(hole) => {
                    let new_line = format!("{} = {}", hole.key, hole.raw_value);
                    buf.push_str(&new_line);
                }
                None => {
                    buf.push_str(line);
                }
            }
        } else {
            buf.push_str(line);
        }

        buf.push('\n')
    }

    buf
}

fn main() {
    let args = Args::parse();

    eprintln!("Repatching from {:?} to {:?}", args.from, args.into);

    let contents: String = std::fs::read_to_string(&args.into).unwrap();
    let holes = parse(&contents);

    let fresh: String = std::fs::read_to_string(args.from).unwrap();
    let res = apply(&fresh, holes);

    println!("{}", res);

    let mut file = File::create(&args.into).unwrap();
    file.write_all(res.as_bytes()).unwrap();
}
