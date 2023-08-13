use std::net::Ipv4Addr;
use clap::Parser;
use rsolver::{Kind, resolve, send_query};
use rsolver::enums::Flag;

#[derive(Parser)]
#[clap(version, name = "rsolver", author = "CarlosEduardoL", about = "Simple DNS resolver CLI utility written in Rust")]
struct Rsolver {
    #[clap(value_name = "DOMAIN")]
    /// The domain to resolve
    domain: String,
    #[clap(long="ns", default_value_t=Ipv4Addr::new(198,41,0,4))]
    /// NameServer IP Address
    name_server: Ipv4Addr,
    /// The record type
    #[clap(long="type", short='t', default_value_t=Kind::A, value_enum)]
    kind: Kind,
    #[clap(short)]
    /// These flags are used to control the behavior of DNS queries and responses.
    flags: Vec<Flag>
}

fn main() {
    let cli = Rsolver::parse();
    let response = resolve(&cli.domain, cli.name_server,cli.kind, &cli.flags);
    println!("{:#?}", response);
}