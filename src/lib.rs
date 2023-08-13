use std::net::{Ipv4Addr, UdpSocket};
use crate::dns_structs::header::{DNSHeader, Flags};
use crate::dns_structs::packet::DNSPacket;
use crate::dns_structs::question::DNSQuestion;
use crate::enums::{Class, Flag};
pub use crate::enums::Kind;

pub mod dns_structs;
pub mod enums;
pub mod errors;

/// Builds a DNS query for the given domain name and record type.
///
/// # Arguments
///
/// * `domain_name` - The domain name to query.
/// * `record_type` - The type of DNS record to query.
/// * `flags` - An array of flags to include in the query.
///
/// # Returns
///
/// A `Vec<u8>` containing the bytes of the DNS query.
fn build_query(domain_name: &str, record_type: Kind, flags: &[Flag]) -> Vec<u8> {
    let id = rand::random();
    let mut query = DNSHeader {
        id,
        flags: Flags::compose(flags),
        num_questions: 1,
        ..DNSHeader::default()
    }.to_bytes();
    query.extend_from_slice(&DNSQuestion {
        name: domain_name.to_string(),
        kind: record_type,
        class: Class::IN,
    }.to_bytes());
    return query;
}

/// Sends a DNS query to the given name server and returns the response.
///
/// # Arguments
///
/// * `domain_name` - The domain name to query.
/// * `name_server` - The IPv4 address of the name server to send the query to.
/// * `record_type` - The type of DNS record to query.
/// * `flags` - An array of flags to include in the query.
///
/// # Returns
///
/// A `Result` containing either a `DNSPacket` representing the response or an error message.
fn send_query(domain_name: &str, name_server: Ipv4Addr, record_type: Kind, flags: &[Flag]) -> Result<DNSPacket, String> {
    let query = build_query(domain_name, record_type, flags);
    let socket = transform_result!(UdpSocket::bind((Ipv4Addr::UNSPECIFIED, 12345)))?;
    transform_result!(socket.connect((name_server, 53)))?;
    transform_result!(socket.send(&query))?;
    let mut answer = vec![0; 2048];
    let _ = transform_result!(socket.recv(&mut answer))?;
    DNSPacket::try_from(answer)
}

/// Resolves the given domain name to an IPv4 address using the given name server and record type.
///
/// # Arguments
///
/// * `domain_name` - The domain name to resolve.
/// * `name_server` - The IPv4 address of the initial name server to use for resolution.
/// * `record_type` - The type of DNS record to use for resolution.
/// * `flags` - An array of flags to include in the queries sent during resolution.
///
/// # Returns
///
/// A `Result` containing either an `Ipv4Addr` representing the resolved IP address or an error message.
pub fn resolve(domain_name: &str, mut name_server: Ipv4Addr, record_type: Kind, flags: &[Flag]) -> Result<Ipv4Addr, String> {
    loop {
        println!("Querying {name_server} for {domain_name}");
        let response = send_query(domain_name, name_server, record_type, flags)?;
        if let Some(ip) = response.get_answer() {
            return Ok(ip)
        } else if let Some(ip) = response.get_name_server_ip() {
            name_server = ip;
        } else if let Some(domain_name) = response.get_name_server() {
            name_server = resolve(&domain_name, name_server, record_type, flags)?;
        } else {
            return Err("Cannot resolve :(".to_string())
        }
    }
}
