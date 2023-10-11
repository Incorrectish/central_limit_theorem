use rand::Rng;
use rayon::prelude::*;
use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Check if at least one argument is provided (the first argument is the program name)
    if args.len() < 4 {
        eprintln!("Usage: <sample_count> <sample_polled> <val1 = prob1, val2 = prob2, ...>");
        std::process::exit(1);
    }

    let n = args[1].parse().expect("Both arguments MUST be integers");
    let total_data_points = args[2].parse().expect("Both arguments MUST be integers");
    let file_path = "arguments.txt";
    let mut args_file = OpenOptions::new()
        .write(true)
        .truncate(true) // This truncates the file if it already exists
        .create(true) // This creates the file if it doesn't exist
        .open(file_path)
        .expect("Could not create data file, due to permissions or something");
    writeln!(args_file, "{}\n{}", n, total_data_points).expect("Unable to write file");

    // Define your map of numbers and their associated probabilities
    let probability_map = parse_string(&args[3]);

    // Normalize probabilities
    let total_prob: f64 = probability_map.iter().map(|&(_, prob)| prob).sum();
    let normalized_probabilities: Vec<(i32, f64)> = probability_map
        .iter()
        .map(|&(num, prob)| (num, prob / total_prob))
        .collect();

    // Create cumulative distribution
    let cumulative_probabilities: Vec<(i32, f64)> = normalized_probabilities
        .iter()
        .scan(0.0, |state, &(num, prob)| {
            *state += prob;
            Some((num, *state))
        })
        .collect();

    let data: Vec<f64> = (0..total_data_points)
        .into_par_iter()
        .map(|_| {
            let mut random_numbers = Vec::new();
            for _ in 0..n {
                let mut rng = rand::thread_rng();
                let rand: f64 = rng.gen(); // Generate a random number between 0 and 1
                for &(num, cumulative_prob) in &cumulative_probabilities {
                    if rand <= cumulative_prob {
                        random_numbers.push(num);
                        break;
                    }
                }
            }
            random_numbers.iter().sum::<i32>() as f64 / n as f64
        })
        .collect();
    let file_path = "data.txt";
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true) // This truncates the file if it already exists
        .create(true) // This creates the file if it doesn't exist
        .open(file_path)
        .expect("Could not create data file, due to permissions or something");
    for value in data {
        writeln!(file, "{}", value).expect("Unable to write to file");
    }
    let python_script = "gen.py";

    let output = Command::new("python3")
        .arg(python_script)
        .output()
        .expect("Failed to run Python script");

    if output.status.success() {
        println!("Python script executed successfully!");
    } else {
        eprintln!("Error: Python script execution failed");
        if let Some(code) = output.status.code() {
            eprintln!("Exit code: {}", code);
        }
        if !output.stderr.is_empty() {
            eprintln!(
                "Error message:\n{}",
                String::from_utf8_lossy(&output.stderr)
            );
        }
    }
}

fn parse_string(input: &str) -> Vec<(i32, f64)> {
    input
        .split(',')
        .map(|pair| {
            let parts: Vec<&str> = pair.trim().split('=').map(|s| s.trim()).collect();
            if parts.len() == 2 {
                if let (Ok(key), Ok(value)) = (parts[0].parse::<i32>(), parts[1].parse::<f64>()) {
                    return (key, value);
                }
            }
            panic!("Invalid input format: {}", pair);
        })
        .collect()
}
