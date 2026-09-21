use clap::Parser;

#[derive(Debug, Parser)]
struct FileScannerArguments {}

fn main() {
    match FileScannerArguments::try_parse() {
        Ok(args) => {
            // entry point in CLI
        },
        Err(error) => {
            println!("error: {}", error);
        }
    }
}
