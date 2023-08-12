use std::io::{Cursor, stdout, Write};
use std::mem::size_of;
use std::net::{Ipv4Addr, UdpSocket};
use crate::dns_structs::header::{DNSHeader, Flags};
use crate::dns_structs::packet::DNSPacket;
use crate::dns_structs::question::DNSQuestion;
use crate::dns_structs::reader::Reader;
use crate::dns_structs::record::DNSRecord;
use crate::enums::{Class, Flag};
pub use crate::enums::Kind;
use crate::random::random;

mod dns_structs;
pub mod enums;
mod random;

fn build_query(domain_name: &str, record_type: Kind, flags: &[Flag]) -> Vec<u8> {
    let id = random();
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

fn send_query(query: Vec<u8>) -> Vec<u8> {
    let socket = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, 12345)).unwrap();
    socket.connect("1.1.1.1:53").unwrap();
    socket.send(&query).unwrap();

    let mut buffer = vec![0; 512];
    let _ = socket.recv(&mut buffer).unwrap();
    buffer
}

pub fn test(domain_name: &str, record_type: Kind, flags: &[Flag]) {
    let query = build_query(domain_name, record_type, flags);
    let answer = send_query(query);
    let packet = DNSPacket::try_from(answer).unwrap();
    println!("{:#?}", packet);
}