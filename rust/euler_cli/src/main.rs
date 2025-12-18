use clap::Parser;
use anyhow::Result;

mod problems;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The problem number to run
    #[arg(short, long)]
    problem: u32,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    println!("Running Problem {}...", args.problem);

    let answer = match problems::solve_problem(args.problem)? {
        Some(answer) => answer,
        None => {
            println!("Problem {} is not implemented yet.", args.problem);
            return Ok(());
        }
    };

    println!("Answer: {}", answer);
    Ok(())
}
