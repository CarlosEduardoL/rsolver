use std::net::Ipv4Addr;
use clap::Parser;
use rsolver::{Kind, resolve, QueryArgs, LogLevel};
use rsolver::enums::Flag;
use rsolver::errors::ResolverResult;

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
    #[clap(long="type", short='t', default_value_t=Kind::ANY, value_enum)]
    kind: Kind,
    #[clap(short)]
    /// These flags are used to control the behavior of DNS queries and responses.
    flags: Vec<Flag>,
    #[clap(short, default_value_t=LogLevel::None, value_enum)]
    /// If true shows all the Queries if false just show the result.
    log_level: LogLevel
}

fn main() -> ResolverResult<()> {
    let cli = Rsolver::parse();
    let args = QueryArgs {
        domain_name: cli.domain,
        name_server: cli.name_server,
        record_type: cli.kind,
        flags: cli.flags,
        log_level: cli.log_level,
    };
    let response = resolve(&args);
    for answer in response? {
        println!("{answer}");
    }
    Ok(())
}
