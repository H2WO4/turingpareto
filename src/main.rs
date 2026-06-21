use clap::{Parser, Subcommand};

mod csv;
mod graph;
mod list;
mod pareto;

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
        // The level ID to operate on
        level:    String,
        // If the level is a programming level, whether to output an animated gif
        #[arg(long, default_value_t = false)]
        animated: bool,
    },
}

fn main() {
    use Commands as C;

    let cli = Cli::parse();

    match &cli.command {
        C::Graph { level, animated } => graph::handle(level, *animated),
        C::List => list::handle(),
    }
}
