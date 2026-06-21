use clap::{Parser, Subcommand};

mod csv;
mod graph;
mod list;
mod pareto;
mod types;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List all available levels
    List,
    /// Generate the graph of a level
    Graph {
        /// The level ID to operate on
        level: String,
        /// Whether to use a log scale for "Gates" and "Ticks"
        #[arg(short, long, default_value_t = false)]
        log:   bool,
    },
}

fn main() {
    use Commands as C;

    let cli = Cli::parse();

    match &cli.command {
        C::Graph { level, log } => graph::handle(level, *log),
        C::List => list::handle(),
    }
}
