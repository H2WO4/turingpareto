use clap::{Parser, Subcommand};

mod csv;
mod list;

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
    Graph { level: String },
}

fn main() {
    use Commands as C;

    let cli = Cli::parse();

    match &cli.command {
        C::Graph { level } => generate_graph(level),
        C::List => list::handle(),
    }
}

fn generate_graph(level: &str) {
    for record in csv::from_level(level) {
        println!("{record:?}")
    }
}
