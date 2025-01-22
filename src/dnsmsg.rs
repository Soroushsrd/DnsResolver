use core::error;

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
    pub fn write(
        &mut self,
        packet: &mut BytePacketBuffer,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.header.questions = self.questions.len() as u16;
        self.header.answers = self.answers.len() as u16;
        self.header.authorative_entries = self.authoritiees.len() as u16;
        self.header.resource_entries = self.resources.len() as u16;

        self.header.write(packet)?;
        for question in &self.questions {
            question.write(packet)?;
        }
        for answer in &self.answers {
            answer.write(packet)?;
        }

        for authority in &self.authoritiees {
            authority.write(packet)?;
        }
        for resource in &self.resources {
            resource.write(packet)?;
        }
        Ok(())
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
