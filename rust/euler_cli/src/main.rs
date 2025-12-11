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

    let answer = match args.problem {
        1 => problems::p0001::solve()?,
        2 => problems::p0002::solve()?,
        3 => problems::p0003::solve()?,
        _ => {
            println!("Problem {} is not implemented yet.", args.problem);
            return Ok(());
        }
    };

    println!("Answer: {}", answer);
    Ok(())
}
