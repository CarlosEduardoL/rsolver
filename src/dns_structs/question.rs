use crate::dns_structs::reader::{NameDecoder, Reader};
use crate::enums::{Class};
use crate::{Kind, transform_result};

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
    type Error = String;

    fn try_from(reader: &mut Reader) -> Result<Self, Self::Error> {
        let name = transform_result!(reader.decode_name())?;

        let kind: u16 = transform_result!("Error reading kind from the response", reader.next_u16())?;
        let class: u16 = transform_result!("Error reading class from the response", reader.next_u16())?;

        Ok(
            Self {
                name,
                kind: Kind::try_from(kind).map_err(|_| "Invalid kind")?,
                class: Class::try_from(class).map_err(|_| "Invalid class")?,
            }
        )
    }
}