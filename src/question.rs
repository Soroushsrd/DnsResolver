use crate::packet::BytePacketBuffer;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum QueryType {
    Unknown(u16),
    A, //1
}

impl From<u16> for QueryType {
    fn from(value: u16) -> Self {
        match value {
            1 => QueryType::A,
            _ => QueryType::Unknown(value),
        }
    }
}

impl From<QueryType> for u16 {
    fn from(value: QueryType) -> Self {
        match value {
            QueryType::A => 1,
            QueryType::Unknown(x) => x,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DnsQuestion {
    pub name: String,
    pub qtype: QueryType,
}

impl DnsQuestion {
    pub fn new(name: String, qtype: QueryType) -> Self {
        Self { name, qtype }
    }

    pub fn read(
        &mut self,
        packet: &mut BytePacketBuffer,
    ) -> Result<(), Box<dyn std::error::Error>> {
        packet.read_qname(&mut self.name)?;
        self.qtype = QueryType::from(packet.read_u16()?);
        let _ = packet.read_u16()?; //class
        Ok(())
    }
}
