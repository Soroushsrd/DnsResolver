use crate::{
    header::DnsHeader,
    packet::BytePacketBuffer,
    question::{DnsQuestion, QueryType},
    record::DnsRecord,
};

#[derive(Debug, Clone)]
pub struct DnsPackets {
    pub header: DnsHeader,
    pub questions: Vec<DnsQuestion>,
    pub answers: Vec<DnsRecord>,
    pub authoritiees: Vec<DnsRecord>,
    pub resources: Vec<DnsRecord>,
}

impl DnsPackets {
    pub fn new() -> DnsPackets {
        DnsPackets {
            header: DnsHeader::new(),
            questions: Vec::new(),
            answers: Vec::new(),
            authoritiees: Vec::new(),
            resources: Vec::new(),
        }
    }
    pub fn from_buffer(
        buffer: &mut BytePacketBuffer,
    ) -> Result<DnsPackets, Box<dyn std::error::Error>> {
        let mut result = DnsPackets::new();
        result.header.read(buffer)?;

        for _ in 0..result.header.questions {
            let mut question = DnsQuestion::new("".to_string(), QueryType::Unknown(0));
            question.read(buffer)?;
            result.questions.push(question);
        }

        for _ in 0..result.header.answers {
            let rec = DnsRecord::read(buffer)?;
            result.answers.push(rec);
        }
        for _ in 0..result.header.authorative_entries {
            let rec = DnsRecord::read(buffer)?;
            result.authoritiees.push(rec);
        }
        for _ in 0..result.header.resource_entries {
            let rec = DnsRecord::read(buffer)?;
            result.resources.push(rec);
        }

        Ok(result)
    }
}
