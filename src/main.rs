use clap::Parser;
use rsolver::{Kind, test};
use rsolver::enums::Flag;

#[derive(Parser)]
#[clap(version, name = "rsolver", author = "CarlosEduardoL", about = "Simple DNS resolver CLI utility written in Rust")]
struct Rsolver {
    #[clap(value_name = "DOMAIN")]
    /// The domain to resolve
    domain: String,
    #[clap(short)]
    /// These flags are used to control the behavior of DNS queries and responses.
    flags: Vec<Flag>
}

fn main() {
    let cli = Rsolver::parse();
    let _ = test(&cli.domain, Kind::A, &cli.flags);
}