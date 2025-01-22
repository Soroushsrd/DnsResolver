use std::{
    net::{Ipv4Addr, Ipv6Addr},
    os::windows::raw,
};

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
    NS {
        domain: String,
        host: String,
        ttl: u32,
    }, //2
    CNAME {
        domain: String,
        host: String,
        ttl: u32,
    }, //5
    MX {
        domain: String,
        priority: u16,
        host: String,
        ttl: u32,
    }, //15
    AAAA {
        domain: String,
        addr: Ipv6Addr,
        ttl: u32,
    }, //28
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
            QueryType::AAAA => {
                let raw_addr1 = packet.read_u32()?;
                let raw_addr2 = packet.read_u32()?;
                let raw_addr3 = packet.read_u32()?;
                let raw_addr4 = packet.read_u32()?;
                let addr = Ipv6Addr::new(
                    ((raw_addr1 >> 16) & 0xFFFF) as u16,
                    ((raw_addr1 >> 0) & 0xFFFF) as u16,
                    ((raw_addr2 >> 16) & 0xFFFF) as u16,
                    ((raw_addr2 >> 0) & 0xFFFF) as u16,
                    ((raw_addr3 >> 16) & 0xFFFF) as u16,
                    ((raw_addr3 >> 0) & 0xFFFF) as u16,
                    ((raw_addr4 >> 16) & 0xFFFF) as u16,
                    ((raw_addr4 >> 0) & 0xFFFF) as u16,
                );
                Ok(DnsRecord::AAAA { domain, addr, ttl })
            }
            QueryType::NS => {
                let mut ns = String::new();
                packet.read_qname(&mut ns)?;

                Ok(DnsRecord::NS {
                    domain,
                    host: ns,
                    ttl,
                })
            }
            QueryType::CNAME => {
                let mut cname = String::new();
                packet.read_qname(&mut cname)?;

                Ok(DnsRecord::CNAME {
                    domain,
                    host: cname,
                    ttl,
                })
            }
            QueryType::MX => {
                let priority = packet.read_u16()?;
                let mut mx = String::new();

                packet.read_qname(&mut mx)?;

                Ok(DnsRecord::MX {
                    domain,
                    priority,
                    host: mx,
                    ttl,
                })
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
    pub fn write(
        &self,
        packet: &mut BytePacketBuffer,
    ) -> Result<usize, Box<dyn std::error::Error>> {
        let start_pos = packet.pos();
        match *self {
            DnsRecord::A {
                ref domain,
                addr,
                ttl,
            } => {
                packet.write_qname(&domain)?;
                packet.write_u16(u16::from(QueryType::A))?;
                packet.write_u16(1)?;
                packet.write_u32(ttl)?;
                packet.write_u16(4)?;

                let octets = addr.octets();
                packet.write(octets[0])?;
                packet.write(octets[1])?;
                packet.write(octets[2])?;
                packet.write(octets[3])?;
            }
            DnsRecord::NS {
                ref domain,
                ref host,
                ttl,
            } => {
                packet.write_qname(&domain)?;
                packet.write_u16(u16::from(QueryType::NS))?;
                packet.write_u16(1)?;
                packet.write_u32(ttl)?;
                // since we dont know the size of host, we will write 16 bits of 0 and set it
                // later!
                let pos = packet.pos();
                packet.write_u16(0)?;

                packet.write_qname(&host)?;

                let size = packet.pos() - (pos + 2);
                // now we set the size
                packet.set_u16(pos, size as u16);
            }
            DnsRecord::CNAME {
                ref domain,
                ref host,
                ttl,
            } => {
                packet.write_qname(domain)?;
                packet.write_u16(u16::from(QueryType::CNAME))?;
                packet.write_u16(1)?;
                packet.write_u32(ttl)?;

                let pos = packet.pos();
                packet.write_u16(0)?;

                packet.write_qname(host)?;

                let size = packet.pos() - (pos + 2);
                packet.set_u16(pos, size as u16);
            }
            DnsRecord::MX {
                ref domain,
                priority,
                ref host,
                ttl,
            } => {
                packet.write_qname(domain)?;
                packet.write_u16(u16::from(QueryType::MX))?;
                packet.write_u16(1)?;
                packet.write_u32(ttl)?;

                let pos = packet.pos();
                packet.write_u16(0)?;

                packet.write_u16(priority)?;
                packet.write_qname(host)?;

                let size = packet.pos() - (pos + 2);
                packet.set_u16(pos, size as u16);
            }
            DnsRecord::AAAA {
                ref domain,
                ref addr,
                ttl,
            } => {
                packet.write_qname(domain)?;
                packet.write_u16(u16::from(QueryType::AAAA))?;
                packet.write_u16(1)?;
                packet.write_u32(ttl)?;
                packet.write_u16(16)?;

                for octet in &addr.segments() {
                    packet.write_u16(*octet)?;
                }
            }
            DnsRecord::Unknown { .. } => {
                println!("Skipping record: {:?}", self);
            }
        }
        Ok(packet.pos - start_pos)
    }
}
