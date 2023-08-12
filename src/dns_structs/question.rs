use std::io::Read;
use crate::dns_structs::reader::{DecodeError, NameDecoder, Reader};
use crate::enums::{Class};
use crate::Kind;

/// DNS Question
///
/// The DNS question section contains one or more questions that are being asked by a client. Each question specifies a domain name, a record type, and a record class.
#[derive(Debug)]
pub struct DNSQuestion {
    /// Domain Name
    ///
    /// A domain name represented as a sequence of labels separated by dots. Each label consists of a length octet followed by that number of octets. The domain name terminates with the zero length octet for the null label of the root.
    pub(crate) name: String,

    /// Record Type
    ///
    /// A two octet code which specifies the type of the query. The values for this field include all codes valid for a `TYPE` field.
    pub(crate) kind: Kind,

    /// Record Class
    ///
    /// A two octet code that specifies the class of the query. For example, the `IN` class is used for Internet addresses.
    pub(crate) class: Class,
}

impl DNSQuestion {
    fn encode_name(&self) -> Vec<u8> {
        self.name.split('.')
            .flat_map(|part| std::iter::once(part.len() as u8).chain(part.bytes()))
            .chain(std::iter::once(0))
            .collect()
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = self.encode_name();
        bytes.extend_from_slice(&(self.kind as u16).to_be_bytes());
        bytes.extend_from_slice(&(self.class as u16).to_be_bytes());
        bytes
    }
}

impl TryFrom<&mut Reader> for DNSQuestion {
    type Error = &'static str;

    fn try_from(reader: &mut Reader) -> Result<Self, Self::Error> {
        let name = match reader.decode_name() {
            Err(DecodeError::Io(_)) => { return Err("An I/O error occurred while decoding the name.") },
            Err(DecodeError::Utf8(_)) => { return Err("The decoded name is not valid UTF-8.") },
            Ok(name) => name
        };

        let kind = reader.next_u16().map_err(|_| "Error reading from the response")?;
        let class = reader.next_u16().map_err(|_| "Error reading from the response")?;

        Ok(
            Self {
                name,
                kind: Kind::try_from(kind).map_err(|_| "Invalid kind")?,
                class: Class::try_from(class).map_err(|_| "Invalid class")?,
            }
        )
    }
}