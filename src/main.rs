use std::env;

#[derive(Debug)]
struct Sizes {
    kb: f64,
    mb: f64,
    gb: f64,
}

enum FileSize {
    Kb(f64),
    Mb(f64),
    Gb(f64),
}

impl Sizes {
    fn new(size: FileSize) -> Sizes {
        match size {
            FileSize::Kb(value) => Sizes {
                kb: value,
                mb: value / 1000.0,
                gb: value / 1_000_000.0,
            },

            FileSize::Mb(value) => Sizes {
                kb: value * 1000.0,
                mb: value,
                gb: value / 1000.0,
            },

            FileSize::Gb(value) => Sizes {
                kb: value * 1_000_000.0,
                mb: value * 1000.0,
                gb: value,
            },
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: cargo run -- \"300 kb\"");
        return;
    }

    let input = &args[1];
    let parts: Vec<&str> = input.split_whitespace().collect();

    if parts.len() != 2 {
        println!("Invalid input. Example: 300 kb");
        return;
    }

    let value: f64 = match parts[0].parse() {
        Ok(value) => value,
        Err(_) => {
            println!("Invalid number.");
            return;
        }
    };

    let size = match parts[1].to_lowercase().as_str() {
        "kb" => FileSize::Kb(value),
        "mb" => FileSize::Mb(value),
        "gb" => FileSize::Gb(value),
        _ => {
            println!("Invalid unit. Use kb, mb or gb.");
            return;
        }
    };

    let sizes = Sizes::new(size);

    println!("{:?}", sizes);
}