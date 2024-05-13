use clap::Parser;
use std::sync::Arc;

/// Universal utility tool for command line programs
#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
pub struct Args {
    /// Command to process
    #[command()]
    pub program: Arc<str>,

    /// How many times to run. Not specifying this parameter causes benchbob to adapt automatically
    #[arg(short)]
    pub n: Option<usize>,

    /// Hard limit the amount of runs
    #[arg(long, default_value_t = 10_000)]
    pub max_runs: usize,

    /// Max runtime in seconds
    #[arg(short, long, default_value_t = 3)]
    pub timeout: u128,

    /// Tells benchbob to measure the whole runtime of the program.
    #[arg(long, default_value_t = false)]
    pub whole_program: bool,
}
