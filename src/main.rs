//! This is a Rust solution to the _Die Agony_ puzzle,
//! described in <https://www.janestreet.com/puzzles/die-agony-index/>.

mod board;
mod dice;
mod direction;
mod solver;

use solver::{Solution, Solver};

use clap::Parser;

#[derive(Parser)]
#[command(version, about)]
struct Args {
    /// Print a textual explanation of the solution, if any is found.
    #[arg(short, long)]
    explain: bool,
}

fn main() {
    let args = Args::parse();
    match Solver::new().solve() {
        Solution::Found(sum_unvisited_cells, explanation) => {
            println!(
                "The sum of values in the unvisited cells is {}.",
                sum_unvisited_cells
            );

            if args.explain {
                println!("{}", explanation);
            }
        }
        Solution::NotFound => println!("Oops, no solution found."),
    }
}
