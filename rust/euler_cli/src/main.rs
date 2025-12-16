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
        4 => problems::p0004::solve()?,
        5 => problems::p0005::solve()?,
        6 => problems::p0006::solve()?,
        7 => problems::p0007::solve()?,
        8 => problems::p0008::solve()?,
        9 => problems::p0009::solve()?,
        10 => problems::p0010::solve()?,
        11 => problems::p0011::solve()?,
        12 => problems::p0012::solve()?,
        13 => problems::p0013::solve()?,
				14 => problems::p0014::solve()?,
        15 => problems::p0015::solve()?,
        16 => problems::p0016::solve()?,
        17 => problems::p0017::solve()?,
        18 => problems::p0018::solve()?,
        19 => problems::p0019::solve()?,
        20 => problems::p0020::solve()?,
        21 => problems::p0021::solve()?,
        22 => problems::p0022::solve()?,
        _ => {
            println!("Problem {} is not implemented yet.", args.problem);
            return Ok(());
        }
    };

    println!("Answer: {}", answer);
    Ok(())
}
