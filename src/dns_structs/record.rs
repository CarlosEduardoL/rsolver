use std::io::Read;
use crate::dns_structs::reader::{DecodeError, NameDecoder, Reader};
use crate::enums::Class;
use crate::Kind;

#[derive(Debug)]
pub struct DNSRecord {
    pub(crate) name: String,
    pub(crate) kind: Kind,
    pub(crate) class: Class,
    pub(crate) ttl: u32,
    pub(crate) data: Vec<u8>,
}

impl TryFrom<&mut Reader> for DNSRecord {
    type Error = &'static str;

    fn try_from(reader: &mut Reader) -> Result<Self, Self::Error> {
        let name = match reader.decode_name() {
            Err(DecodeError::Io(_)) => { return Err("An I/O error occurred while decoding the name."); }
            Err(DecodeError::Utf8(_)) => { return Err("The decoded name is not valid UTF-8."); }
            Ok(name) => name
        };

        let kind: u16 = reader.next_u16().map_err(|_| "Error reading kind from the response")?;
        let class: u16 = reader.next_u16().map_err(|_| "Error reading class from the response")?;
        let ttl: u32 = reader.next_u32().map_err(|_| "Error reading ttl from the response")?;
        let data_len: u16 = reader.next_u16().map_err(|_| "Error reading data len from the response")?;

        let mut data = vec![0; data_len as usize];
        reader.read_exact(&mut data).map_err(|_| "Error reading data from the response")?;

        Ok(
            Self {
                name,
                kind: Kind::try_from(kind).map_err(|_| "Invalid kind")?,
                class: Class::try_from(class).map_err(|_| "Invalid class")?,
                ttl,
                data,
            }
        )
    }
}