use std::fmt::{Debug, Formatter};
use std::io::{Read};
use std::mem::size_of;
use strum::IntoEnumIterator;
use crate::dns_structs::reader::Reader;
use crate::enums::Flag;
use crate::transform_result;

#[repr(transparent)]
#[derive(Copy, Clone, Default)]
pub struct Flags(pub u16);

impl Flags {
    /// Creates a new `Flag` by composing the given flags using the bitwise OR operation.
    pub fn compose(flags: &[Flag]) -> Self {
        let value = flags.iter().fold(0, |acc, flag| acc | *flag as u16);
        Self(value)
    }
}

impl Debug for Flags {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", Flag::iter().filter(|flag| *flag as u16 & self.0 == *flag as u16).collect::<Vec<_>>())
    }
}

/// DNS Header
///
/// The DNS header contains information about the DNS message, including its type (query or response), flags, and the number of records in each section of the message.
#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct DNSHeader {
    /// Identifier
    ///
    /// A 16-bit identifier assigned by the program that generates any kind of query. This identifier is copied to the corresponding reply and can be used by the requester to match up replies to outstanding queries.
    pub(crate) id: u16,

    /// Flags
    ///
    /// A 16-bit field containing various flags that control the behavior of the query and response. See the `Flags` enum for more information.
    pub(crate) flags: Flags,

    /// Number of Questions
    ///
    /// An unsigned 16-bit integer specifying the number of entries in the question section.
    pub(crate) num_questions: u16,

    /// Number of Answers
    ///
    /// An unsigned 16-bit integer specifying the number of resource records in the answer section.
    pub(crate) num_answers: u16,

    /// Number of Authorities
    ///
    /// An unsigned 16-bit integer specifying the number of name server resource records in the authority records section.
    pub(crate) num_authorities: u16,

    /// Number of Additionals
    ///
    /// An unsigned 16-bit integer specifying the number of resource records in the additional records section.
    pub(crate) num_additionals: u16,
}

impl DNSHeader {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![0; size_of::<Self>()];
        unsafe {
            std::ptr::copy_nonoverlapping(
                self as *const Self as *const u8,
                bytes.as_mut_ptr(),
                size_of::<Self>(),
            );
        }
        if cfg!(target_endian = "little") {
            bytes.chunks_exact_mut(2).for_each(|chunk| chunk.swap(0, 1));
        }
        bytes
    }
}

impl TryFrom<&mut Reader> for DNSHeader {
    type Error = String;

    fn try_from(reader: &mut Reader) -> Result<Self, Self::Error> {
        let mut buffer = [0u8; size_of::<Self>()];
        transform_result!("Cannot read the header from the response", reader.read_exact(&mut buffer))?;

        let id = u16::from_be_bytes([buffer[0], buffer[1]]);
        let flags = Flags(u16::from_be_bytes([buffer[2], buffer[3]]));
        let num_questions = u16::from_be_bytes([buffer[4], buffer[5]]);
        let num_answers = u16::from_be_bytes([buffer[6], buffer[7]]);
        let num_authorities = u16::from_be_bytes([buffer[8], buffer[9]]);
        let num_additionals = u16::from_be_bytes([buffer[10], buffer[11]]);

        Ok(Self {
            id,
            flags,
            num_questions,
            num_answers,
            num_authorities,
            num_additionals,
        })
    }
}