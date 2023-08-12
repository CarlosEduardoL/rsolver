use crate::dns_structs::header::DNSHeader;
use crate::dns_structs::question::DNSQuestion;
use crate::dns_structs::reader::Reader;
use crate::dns_structs::record::DNSRecord;

#[derive(Debug)]
pub struct DNSPacket {
    header: DNSHeader,
    questions: Vec<DNSQuestion>,
    answers: Vec<DNSRecord>,
    authorities: Vec<DNSRecord>,
    additionals: Vec<DNSRecord>,
}

impl TryFrom<Vec<u8>> for DNSPacket {
    type Error = &'static str;

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
                .collect::<Result<Vec<_>, _>>()?
        })
    }
}