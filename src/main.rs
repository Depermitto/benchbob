use clap::Parser;
use std::process::Command;
use std::time::Instant;

/// Universal utility tool for command line programs
#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct Args {
    /// Command to process
    #[arg(short, long)]
    program: String,
}

fn main() {
    let args = Args::parse();

    let mut program_parts = args.program.split_whitespace();
    let Some(program) = program_parts.next() else {
        println!("no work to do");
        return;
    };

    let start = Instant::now();
    match Command::new(program).args(program_parts).spawn() {
        Ok(mut child) => {
            let _ = child.wait();
            println!("\nTime taken: {:?}", start.elapsed());
        },
        Err(_) => println!("invalid command"),
    }
}
