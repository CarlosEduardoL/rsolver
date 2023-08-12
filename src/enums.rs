use clap::ValueEnum;
use derive_try_from_primitive::TryFromPrimitive;

/// This is a Rust implementation of the DNS record types defined in RFC 1035 section 3.2.2.
/// The `Kind` enum represents the different types of DNS records, with each variant corresponding to a specific record type.
/// The values assigned to each variant represent the numerical value of the record type as defined in the RFC.
#[repr(u16)]
#[derive(Copy, Clone, Default, Debug)]
#[derive(TryFromPrimitive)]
pub enum Kind {
    #[default]
    /// An A record type, used for storing an IP address.
    A = 1,
    /// An authoritative name-server record type.
    NS = 2,
    /// A mail-destination record type (Obsolete).
    MD = 3,
    /// A mail forwarder record type (Obsolete).
    MF = 4,
    /// A record type that contains the canonical name of a DNS alias.
    CNAME = 5,
    /// A Start of Authority (SOA) record type.
    SOA = 6,
    /// A mailbox record type (EXPERIMENTAL).
    MB = 7,
    /// A mail group member record type (EXPERIMENTAL).
    MG = 8,
    /// A mail-rename record type (EXPERIMENTAL).
    MR = 9,
    /// A record type for completion queries (EXPERIMENTAL).
    NULL = 10,
    /// A record type for a well-known service.
    WKS = 11,
    /// A record type containing FQDN pointer.
    PTR = 12,
    /// A host information record type.
    HINFO = 13,
    /// A mailbox or mailing list information record type (EXPERIMENTAL).
    MINFO = 14,
    /// A mail-exchanger record type.
    MX = 15,
    /// A record type containing a text string.
    TXT = 16,
}

/// This is a Rust implementation of the DNS class types defined in RFC 1035 section 3.2.4.
/// The `Class` enum represents the different classes of DNS records, with each variant corresponding to a specific class type.
/// The values assigned to each variant represent the numerical value of the class type as defined in the RFC.
#[repr(u16)]
#[derive(TryFromPrimitive)]
#[derive(Copy, Clone, Default, Debug)]
pub enum Class {
    #[default]
    /// The Internet class.
    IN = 1,
    /// The CSNET class (Obsolete - used only for examples in some obsolete RFCs).
    CS = 2,
    /// The CHAOS class.
    CH = 3,
    /// Hesiod [Dyer 87].
    HS = 4,
}

/// DNS Header Flags
///
/// These flags are used to control the behavior of DNS queries and responses.
/// They are contained in the DNS header and are 16 bits long.
///
/// For more information, see [IANA's website](https://www.iana.org/assignments/dns-parameters/dns-parameters.xhtml).
#[repr(u16)]
#[derive(Copy, Clone, ValueEnum, Debug)]
pub enum Flag {
    /// Authoritative Answer (AA)
    ///
    /// This 1-bit flag is set to 1 in a response if the responding server is an authority for the domain name used in the question.
    AA = 1 << 5,

    /// Truncated Response (TC)
    ///
    /// This 1-bit flag is set to 1 if the message was truncated due to its length exceeding the maximum allowed by the underlying transport protocol.
    TC = 1 << 6,

    /// Recursion Desired (RD)
    ///
    /// This 1-bit flag is set to 1 in a query if the client wants the server to perform recursion to resolve the query.
    RD = 1 << 7,

    /// Recursion Available (RA)
    ///
    /// This 1-bit flag is set to 1 in a response if the server supports recursion.
    RA = 1 << 8,

    /// Authentic Data (AD)
    ///
    /// This 1-bit flag is set to 1 in a response if all the data included in the answer and authority sections of the response have been authenticated by the server according to the policies of that server.
    AD = 1 << 10,

    /// Checking Disabled (CD)
    ///
    /// This 1-bit flag is set to 1 in a query to disable checking of data authenticity by the server.
    CD = 1 << 11,
}