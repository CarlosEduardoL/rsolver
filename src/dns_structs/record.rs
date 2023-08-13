use std::io::Read;
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::dns_structs::reader::{NameDecoder, Reader};
use crate::dns_structs::record::Data::{A, AAAA, HostName, Other};
use crate::enums::Class;
use crate::{Kind, transform_result};

#[derive(Debug, Clone)]
pub enum Data {
    HostName(String),
    A(Ipv4Addr),
    AAAA(Ipv6Addr),
    Other(Vec<u8>),
}

#[derive(Debug, Clone)]
pub struct DNSRecord {
    pub name: String,
    pub kind: Kind,
    pub class: Class,
    pub ttl: u32,
    pub data: Data,
}

impl TryFrom<&mut Reader> for DNSRecord {
    type Error = String;

    fn try_from(reader: &mut Reader) -> Result<Self, Self::Error> {
        let name = transform_result!(reader.decode_name())?;

        let kind: u16 = transform_result!("Error reading kind from the response", reader.next_u16())?;
        let class: u16 = transform_result!("Error reading class from the response", reader.next_u16())?;
        let ttl: u32 = transform_result!("Error reading ttl from the response",reader.next_u32())?;
        let data_len: u16 = transform_result!("Error reading data len from the response",reader.next_u16())?;

        let kind = Kind::try_from(kind).map_err(|_| "Invalid kind")?;

        let data = match kind {
            Kind::NS => HostName(transform_result!(reader.decode_name())?),
            Kind::A => {
                assert_eq!(data_len, 4);
                let mut ip = [0u8; 4];
                transform_result!("Error reading ip from the response",reader.read_exact(&mut ip))?;
                A(Ipv4Addr::from(ip))
            }
            Kind::AAAA => {
                assert_eq!(data_len, 16);
                let mut ip = [0u8; 16];
                transform_result!("Error reading ip from the response",reader.read_exact(&mut ip))?;
                AAAA(Ipv6Addr::from(ip))
            }
            _ => {
                let mut data = vec![0; data_len as usize];
                transform_result!("Error reading data from the response",reader.read_exact(&mut data))?;
                Other(data)
            }
        };

        Ok(
            Self {
                name,
                kind,
                class: Class::try_from(class).map_err(|_| "Invalid class")?,
                ttl,
                data,
            }
        )
    }
}