use clap::Parser;

#[derive(Parser)]
#[clap(version, name = "rsolver", author = "CarlosEduardoL", about = "Simple DNS resolver CLI utility written in Rust")]
struct Rsolver {
    #[clap(value_name="DOMAIN")]
    domain: String
}

fn main() {
    let cli = Rsolver::parse();
    println!("{}", cli.domain);
}
