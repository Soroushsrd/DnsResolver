use std::net::Ipv4Addr;

use crate::{packet::BytePacketBuffer, question::QueryType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DnsRecord {
    Unknown {
        domain: String,
        qtype: u16,
        data_len: u16,
        ttl: u32,
    },
    A {
        domain: String,
        addr: Ipv4Addr,
        ttl: u32,
    }, //1
}

impl DnsRecord {
    pub fn read(packet: &mut BytePacketBuffer) -> Result<DnsRecord, Box<dyn std::error::Error>> {
        let mut domain = String::new();
        packet.read_qname(&mut domain)?;

        let qtype = QueryType::from(packet.read_u16()?);

        let _ = packet.read_u16()?;
        let ttl = packet.read_u32()?;
        let data_len = packet.read_u16()?;

        match qtype {
            QueryType::A => {
                let raw_addr = packet.read_u32()?;
                let addr = Ipv4Addr::new(
                    ((raw_addr >> 24) & 0xFF) as u8,
                    ((raw_addr >> 16) & 0xFF) as u8,
                    ((raw_addr >> 8) & 0xFF) as u8,
                    ((raw_addr >> 0) & 0xFF) as u8,
                );
                Ok(DnsRecord::A { domain, addr, ttl })
            }
            QueryType::Unknown(_) => {
                packet.step(data_len as usize);
                Ok(DnsRecord::Unknown {
                    domain,
                    qtype: qtype.into(),
                    data_len,
                    ttl,
                })
            }
        }
    }
}
