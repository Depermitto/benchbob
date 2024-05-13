use crate::args::Args;
use clap::Parser;
use std::process::{Command, ExitCode, Stdio};
use std::time::{Duration, Instant};
use textplots::{utils, Chart, LabelBuilder, LabelFormat, Plot, Shape};

mod args;

fn main() -> ExitCode {
    let args = Args::parse();
    let max_runtime_nanos = Duration::from_secs(args.timeout);

    let mut program_parts = args.program.trim().split_whitespace();
    let Some(program) = program_parts.next() else {
        println!("No work to do");
        return ExitCode::from(1);
    };

    let mut times: Vec<Duration> = Vec::with_capacity(args.n.unwrap_or(args.max_runs));
    let mut total_runtime = Duration::ZERO;
    for _ in 0..times.capacity() {
        if total_runtime > max_runtime_nanos {
            break;
        }

        let cmd_start = Instant::now();
        let cmd = Command::new(program)
            .args(program_parts.clone())
            .stderr(Stdio::null())
            .stdout(Stdio::null())
            .spawn();

        match cmd {
            Ok(mut child) => {
                let _ = child.wait();
                let runtime = cmd_start.elapsed();
                total_runtime += runtime;
                times.push(runtime);
            }
            Err(_) => {
                println!("Invalid command");
                return ExitCode::from(1);
            }
        }
    }
    assert!(!times.is_empty());

    times.sort();

    let mean = times
        .iter()
        .fold(Duration::ZERO, |acc, e| acc + *e)
        .div_f64(times.len() as f64);

    let median = if times.len() % 2 == 1 {
        times[times.len() / 2 - 1]
    } else {
        times[times.len() / 2] + times[times.len() / 2 - 1]
    };

    let (min, max) = (times[0], times[times.len() - 1]);

    let points = times
        .iter()
        .enumerate()
        .map(|(i, time)| (i as f32, time.as_nanos() as f32))
        .collect::<Vec<(f32, f32)>>();
    let hist = utils::histogram(
        &points,
        min.as_nanos() as f32,
        max.as_nanos() as f32,
        times.len(),
    );

    Chart::new(180, 60, min.as_nanos() as f32, max.as_nanos() as f32)
        .lineplot(&Shape::Bars(&hist))
        .x_label_format(LabelFormat::Custom(Box::new(|ns| {
            format!("{:.2?}", Duration::from_nanos(ns as u64))
        })))
        .display();

    println!(
        "Total runs = {}
Min = {:.2?}
Max = {:.2?}
Average = {:.2?}
Median = {:.2?}
Time taken = {:.2?}",
        times.len(),
        min,
        max,
        median,
        mean,
        total_runtime,
    );

    ExitCode::from(0)
}
