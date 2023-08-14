use std::fmt::{Display, Formatter};
use std::io::Read;
use std::net::{Ipv4Addr, Ipv6Addr};
use crate::dns_structs::reader::{NameDecoder, Reader};
use crate::dns_structs::record::Data::{A, AAAA, NS, Other};
use crate::enums::Class;
use crate::{Kind, transform_result};

/// An enumeration representing the different types of data that can be stored in a DNS record.
#[derive(Debug, Clone)]
pub enum Data {
    /// A host name.
    NS(String),
    /// An IPv4 address.
    A(Ipv4Addr),
    /// An IPv6 address.
    AAAA(Ipv6Addr),
    /// Other data.
    Other(Vec<u8>),
}

impl Display for Data {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            NS(hostname) => write!(f, "{}", hostname),
            A(ip) => write!(f, "{}", ip),
            AAAA(ip) => write!(f, "{}", ip),
            Other(raw_data) => write!(f, "{}", String::from_utf8_lossy(raw_data))
        }
    }
}

/// A structure representing a DNS record.
#[derive(Debug, Clone)]
pub struct DNSRecord {
    /// The name associated with the record.
    pub name: String,
    /// The type of the record.
    pub kind: Kind,
    /// The class of the record.
    pub class: Class,
    /// The time-to-live value of the record.
    pub ttl: u32,
    /// The data associated with the record.
    pub data: Data,
}

impl TryFrom<&mut Reader> for DNSRecord {
    type Error = String;

    /// Attempts to create a `DNSRecord` from the given `Reader`.
    ///
    /// # Arguments
    ///
    /// * `reader` - A mutable reference to a `Reader` containing the bytes of a DNS record.
    ///
    /// # Returns
    ///
    /// A `Result` containing either a `DNSRecord` or an error message.
    fn try_from(reader: &mut Reader) -> Result<Self, Self::Error> {
        let name = transform_result!(reader.decode_name())?;

        let kind: u16 = transform_result!("Error reading kind from the response", reader.next_u16())?;
        let class: u16 = transform_result!("Error reading class from the response", reader.next_u16())?;
        let ttl: u32 = transform_result!("Error reading ttl from the response",reader.next_u32())?;
        let data_len: u16 = transform_result!("Error reading data len from the response",reader.next_u16())?;

        let kind = Kind::try_from(kind).map_err(|_| "Invalid kind")?;

        let data = match kind {
            Kind::NS => NS(transform_result!(reader.decode_name())?),
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
