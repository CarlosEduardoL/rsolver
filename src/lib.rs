use std::net::{Ipv4Addr, UdpSocket};
use crate::dns_structs::header::{DNSHeader, Flags};
use crate::dns_structs::packet::DNSPacket;
use crate::dns_structs::question::DNSQuestion;
use crate::enums::{Class, Flag};
pub use crate::enums::Kind;

pub mod dns_structs;
pub mod enums;
pub mod errors;

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

pub fn send_query(domain_name: &str, name_server: Ipv4Addr, record_type: Kind, flags: &[Flag]) -> Result<DNSPacket, String> {
    let query = build_query(domain_name, record_type, flags);
    let socket = transform_result!(UdpSocket::bind((Ipv4Addr::UNSPECIFIED, 12345)))?;
    transform_result!(socket.connect((name_server, 53)))?;
    transform_result!(socket.send(&query))?;
    let mut answer = vec![0; 2048];
    let _ = transform_result!(socket.recv(&mut answer))?;
    DNSPacket::try_from(answer)
}

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