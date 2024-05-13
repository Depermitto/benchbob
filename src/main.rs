use crate::args::Args;
use clap::Parser;
use std::process::{Command, ExitCode, Stdio};
use std::time::{Duration, Instant};
use textplots::{utils, Chart, LabelBuilder, LabelFormat, Plot, Shape};

mod args;

fn main() -> ExitCode {
    let args = Args::parse();
    let max_runtime_nanos = args.timeout * 1e9 as u128;

    let mut program_parts = args.program.trim().split_whitespace();
    let Some(program) = program_parts.next() else {
        println!("no work to do");
        return ExitCode::from(1);
    };

    let mut times: Vec<Duration> = Vec::with_capacity(args.n.unwrap_or(args.max_runs));
    let mut total_runtime: u128 = 0;
    for _ in 0..times.capacity() {
        if total_runtime > max_runtime_nanos {
            break;
        }

        let cmd_start = Instant::now();
        let cmd = Command::new(program)
            .args(program_parts.clone())
            .stdout(Stdio::null())
            .spawn();

        match cmd {
            Ok(mut child) => {
                let _ = child.wait();
                let runtime = cmd_start.elapsed();
                total_runtime += runtime.as_nanos();
                times.push(runtime);
            }
            Err(_) => {
                println!("invalid command");
                return ExitCode::from(1);
            }
        }
    }

    if times.is_empty() {
        println!("No elements found");
        return ExitCode::from(1);
    }

    let mean = times.iter().fold(0, |acc, e| acc as u128 + e.as_nanos()) as usize / times.len();

    let length = times.len();
    // Compute median by partially sorting the array
    let median = if length % 2 == 1 {
        let (_, mid, _) = times.select_nth_unstable(length / 2);
        mid.as_nanos()
    } else {
        let (_, mid_right, _) = times.select_nth_unstable(length / 2);
        let mid_right = *mid_right;
        let (_, mid_left, _) = times.select_nth_unstable(length / 2 - 1);
        (mid_right.as_nanos() + mid_left.as_nanos()) / 2
    };

    // Compute (min, max) bounds for plotting
    let (min, max) = (times.iter().min().unwrap(), times.iter().max().unwrap());
    let (min, max) = (min.as_nanos() as f32, max.as_nanos() as f32);

    let points = times
        .iter()
        .enumerate()
        .map(|(i, time)| (i as f32, time.as_nanos() as f32))
        .collect::<Vec<(f32, f32)>>();
    let hist = utils::histogram(&points, min, max, length);

    Chart::new(180, 60, min, max)
        .lineplot(&Shape::Bars(&hist))
        .x_label_format(LabelFormat::Custom(Box::new(|ns| {
            format!("{:?}", Duration::from_nanos(ns as u64))
        })))
        .display();

    println!(
        "Total runs = {}
Min = {:?}
Max = {:?}
Average = {:?}
Median = {:?}
Time taken = {:?}",
        length,
        Duration::from_nanos(min as u64),
        Duration::from_nanos(max as u64),
        Duration::from_nanos(median as u64),
        Duration::from_nanos(mean as u64),
        Duration::from_nanos(total_runtime as u64),
    );

    ExitCode::from(0)
}
