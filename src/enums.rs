use clap::ValueEnum;
use derive_try_from_primitive::TryFromPrimitive;
use strum::EnumIter;

/// This is a Rust implementation of the DNS record types defined in RFC 1035 section 3.2.2.
/// Extended using https://en.wikipedia.org/wiki/List_of_DNS_record_types
/// The `Kind` enum represents the different types of DNS records, with each variant corresponding to a specific record type.
/// The values assigned to each variant represent the numerical value of the record type as defined in the RFC.
#[repr(u16)]
#[derive(Copy, Clone, Default, Debug, ValueEnum, Eq, PartialEq)]
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
    /// Responsible Person - RFC 1183
    RP = 17,
    /// AFS database record - RFC 1183
    AFSDB = 18,
    /// Signature - RFC 2535
    SIG = 24,
    /// Key record - RFC 2535[3] and RFC 2930[4]
    KEY = 25,
    /// IPv6 address record - RFC 3596[2]
    AAAA = 28,
    /// Location record - RFC 1876
    LOC = 29,
    /// Service locator - RFC 2782
    SRV = 33,
    /// Naming Authority Pointer - RFC 3403
    NAPTR = 35,
    /// Key Exchanger record - RFC 2230
    KX = 36,
    /// Certificate record - RFC 4398
    CERT = 37,
    /// Delegation name record - RFC 6672
    DNAME = 39,
    /// Address Prefix List - RFC 3123
    APL = 42,
    /// Delegation signer - RFC 4034
    DS = 43,
    /// SSH Public Key Fingerprint - RFC 4255
    SSHFP = 44,
    /// IPsec Key - RFC 4025
    IPSECKEY = 45,
    /// DNSSEC signature - RFC 4034
    RRSIG = 46,
    /// Next Secure record - RFC 4034
    NSEC = 47,
    /// DNS Key record - RFC 4034
    DNSKEY = 48,
    /// DHCP identifier - RFC 4701
    DHCID = 49,
    /// Next Secure record version 3 - RFC 5155
    NSEC3 = 50,
    /// NSEC3 parameters - RFC 5155
    NSEC3PARAM = 51,
    /// TLSA certificate association - RFC 6698
    TLSA = 52,
    /// S/MIME cert association[10] - RFC 8162[9]
    SMIMEA = 53,
    /// Host Identity Protocol - RFC 8005
    HIP = 55,
    /// Child DS - RFC 7344
    CDS = 59,
    /// Child copy of DNSKEY record, for transfer to parent - RFC 7344
    CDNSKEY = 60,
    /// OpenPGP public key record - RFC 7929
    OPENPGPKEY = 61,
    /// Child-to-Parent Synchronization - RFC 7477
    CSYNC = 62,
    /// Message Digests for DNS Zones - RFC 8976
    ZONEMD = 63,
    /// Service Binding - IETF Draft
    SVCB = 64,
    /// HTTPS Binding - IETF Draft
    HTTPS = 65,
    /// MAC address (EUI-48) - RFC 7043
    EUI48 = 108,
    /// MAC address (EUI-64) - RFC 7043
    EUI64 = 109,
    /// Transaction Key record - RFC 2930
    TKEY = 249,
    /// Transaction Signature - RFC 2845
    TSIG = 250,
    /// Uniform Resource Identifier - RFC 7553
    URI = 256,
    /// Certification Authority Authorization - RFC 6844
    CAA = 257,
    /// DNSSEC Trust Authorities
    TA = 32768,
    /// DNSSEC Lookaside Validation record - RFC 4431
    DLV = 32769,
    /// Don't filter any response
    ANY,
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
#[derive(Copy, Clone, ValueEnum, Debug, EnumIter)]
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