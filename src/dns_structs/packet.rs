use std::net::Ipv4Addr;
use crate::dns_structs::header::DNSHeader;
use crate::dns_structs::question::DNSQuestion;
use crate::dns_structs::reader::Reader;
use crate::dns_structs::record::{Data, DNSRecord};
use crate::Kind;

#[derive(Debug)]
pub struct DNSPacket {
    pub header: DNSHeader,
    pub questions: Vec<DNSQuestion>,
    pub answers: Vec<DNSRecord>,
    pub authorities: Vec<DNSRecord>,
    pub additionals: Vec<DNSRecord>,
}

impl DNSPacket {
    pub fn get_answer(&self) -> Option<Ipv4Addr> {
        self.answers
            .iter()
            .filter(|answer| answer.kind == Kind::A)
            .next()
            .map(|answer| match answer.data {
                Data::A(ip) => ip,
                _ => unreachable!()
            })
    }
    
    pub fn get_name_server_ip(&self) -> Option<Ipv4Addr> {
        self.additionals
            .iter()
            .filter(|answer| answer.kind == Kind::A)
            .next()
            .map(|answer| match answer.data {
                Data::A(ip) => ip,
                _ => unreachable!()
            })
    }

    pub fn get_name_server(&self) -> Option<String> {
        self.authorities
            .iter()
            .filter(|answer| answer.kind == Kind::NS)
            .next()
            .map(|answer| match &answer.data {
                Data::HostName(host) => host.clone(),
                _ => unreachable!()
            })
    }
}

impl TryFrom<Vec<u8>> for DNSPacket {
    type Error = String;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        let mut reader = Reader::new(value);
        let header = DNSHeader::try_from(&mut reader)?;
        Ok(Self {
            header,
            questions: (0..header.num_questions)
                .map(|_| DNSQuestion::try_from(&mut reader))
                .collect::<Result<Vec<_>, _>>()?,
            answers: (0..header.num_answers)
                .map(|_| DNSRecord::try_from(&mut reader))
                .collect::<Result<Vec<_>, _>>()?,
            authorities: (0..header.num_authorities)
                .map(|_| DNSRecord::try_from(&mut reader))
                .collect::<Result<Vec<_>, _>>()?,
            additionals: (0..header.num_additionals)
                .map(|_| DNSRecord::try_from(&mut reader))
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}