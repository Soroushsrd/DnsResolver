use crate::packet::BytePacketBuffer;

//ID	Name	Description	                                                Encoding
//1	A	Alias - Mapping names to IP addresses	                        Preamble + Four bytes for IPv4 adress
//2	NS	Name Server - The DNS server address for a domain	        Preamble + Label Sequence
//5	CNAME	Canonical Name - Maps names to names	                        Preamble + Label Sequence
//15	MX	Mail eXchange - The host of the mail server for a domain	Preamble + 2-bytes for priority + Label Sequence
//28	AAAA	IPv6 alias	                                                Premable + Sixteen bytes for IPv6 adress

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum QueryType {
    Unknown(u16),
    A,     //1
    NS,    //2
    CNAME, //5
    MX,    //15
    AAAA,  //28
}

impl From<u16> for QueryType {
    fn from(value: u16) -> Self {
        match value {
            1 => QueryType::A,
            2 => QueryType::NS,
            5 => QueryType::CNAME,
            15 => QueryType::MX,
            28 => QueryType::AAAA,
            _ => QueryType::Unknown(value),
        }
    }
}

impl From<QueryType> for u16 {
    fn from(value: QueryType) -> Self {
        match value {
            QueryType::A => 1,
            QueryType::NS => 2,
            QueryType::CNAME => 5,
            QueryType::MX => 15,
            QueryType::AAAA => 28,
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

    pub fn write(&self, packet: &mut BytePacketBuffer) -> Result<(), Box<dyn std::error::Error>> {
        packet.write_qname(&self.name)?;
        let numbtype = u16::from(self.qtype);
        packet.write_u16(numbtype)?;
        packet.write_u16(1)?;
        Ok(())
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
