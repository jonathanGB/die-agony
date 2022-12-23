//! This is a Rust solution to the _Die Agony_ puzzle,
//! described in <https://www.janestreet.com/puzzles/die-agony-index/>.

use clap::Parser;

mod board;
mod dice;
mod direction;
mod solver;

fn main() {
    solver::Solver::new().solve(solver::SolverArgs::parse());
}
