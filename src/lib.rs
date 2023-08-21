use std::net::{Ipv4Addr, UdpSocket};
use crate::dns_structs::header::{DNSHeader, Flags};
use crate::dns_structs::packet::DNSPacket;
use crate::dns_structs::question::DNSQuestion;
use crate::dns_structs::record::Data;
use crate::enums::{Class, Flag};
use clap::ValueEnum;
pub use crate::enums::Kind;

pub mod dns_structs;
pub mod enums;
pub mod errors;

#[repr(u8)]
#[derive(Copy, Clone, ValueEnum, Debug, PartialEq, Eq)]
pub enum LogLevel {
    None = 0,
    Info = 1,
    Debug = 2
}

/// A structure representing the arguments for a DNS query.
#[derive(Debug, Clone)]
pub struct QueryArgs {
    /// The domain name to query.
    pub domain_name: String,
    /// The IPv4 address of the name server to send the query to.
    pub name_server: Ipv4Addr,
    /// The type of DNS record to query.
    pub record_type: Kind,
    /// An array of flags to include in the query.
    pub flags: Vec<Flag>,
    /// Whether to print debug information.
    pub log_level: LogLevel,
}

/// Builds a DNS query for the given domain name and record type.
///
/// # Arguments
///
/// * `args` - A `QueryArgs` structure containing the arguments for the query.
///
/// # Returns
///
/// A `Vec<u8>` containing the bytes of the DNS query.
fn build_query(args: &QueryArgs) -> Vec<u8> {
    let id = rand::random();
    let mut query = DNSHeader {
        id,
        flags: Flags::compose(&args.flags),
        num_questions: 1,
        ..DNSHeader::default()
    }.to_bytes();
    query.extend_from_slice(&DNSQuestion {
        name: args.domain_name.to_string(),
        kind: args.record_type,
        class: Class::IN,
    }.to_bytes());
    return query;
}

/// Sends a DNS query to the given name server and returns the response.
///
/// # Arguments
///
/// * `args` - A `QueryArgs` structure containing the arguments for the query.
///
/// # Returns
///
/// A `Result` containing either a `DNSPacket` representing the response or an error message.
fn send_query(args: &QueryArgs) -> Result<DNSPacket, String> {
    let query = build_query(args);
    let socket = transform_result!(UdpSocket::bind((Ipv4Addr::UNSPECIFIED, 12345)))?;
    transform_result!(socket.connect((args.name_server, 53)))?;
    transform_result!(socket.send(&query))?;
    let mut answer = vec![0; 2048];
    let _ = transform_result!(socket.recv(&mut answer))?;
    DNSPacket::try_from(answer)
}

/// Resolves the given domain name to an IPv4 address using the given name server and record type.
///
/// # Arguments
///
/// * `args` - A `QueryArgs` structure containing the arguments for the resolution.
///
/// # Returns
///
/// A `Result` containing either an `Ipv4Addr` representing the resolved IP address or an error message.
pub fn resolve(args: &QueryArgs) -> Result<Vec<Data>, String> {
    let mut args = args.clone();
    loop {
        if args.log_level as u8 >= LogLevel::Info as u8 {
            println!("Querying {} for {}", &args.name_server, args.domain_name);
        }
        if args.log_level as u8 >= LogLevel::Debug as u8 {
            println!("{args:?}");
        }
        let response = send_query(&args)?;
        if args.log_level as u8 >= LogLevel::Debug as u8 {
            println!("{response:#?}");
        }
        let answers = response.get_answers(args.record_type);
        if !answers.is_empty() {
            return Ok(answers)
        } else if let Some(ip) = response.get_name_server_ip() {
            args.name_server = ip;
        } else if let Some(domain_name) = response.get_name_server() {
            let new_args = QueryArgs {
                domain_name: domain_name.clone(),
                record_type: Kind::A,
                ..args.clone()
            };
            let result = resolve(&new_args)?;
            args.name_server = match result.into_iter().next() {
                Some(Data::IPv4(ip)) => ip,
                _ => { return transform_result!(Err("This will never happens")) }
            };
        } else {
            return Err(format!("Cannot resolve {} on server {}", &args.domain_name, &args.name_server))
        }
    }
}