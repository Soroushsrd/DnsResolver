use std::{fs::File, io::Read};

use dnsmsg::DnsPackets;
use packet::BytePacketBuffer;

pub mod dnsmsg;
pub mod header;
pub mod packet;
pub mod question;
pub mod record;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut f = File::open("response_packet.txt")?;
    let mut buffer = BytePacketBuffer::new();
    f.read(&mut buffer.buff)?;

    let packet = DnsPackets::from_buffer(&mut buffer)?;
    println!("{:#?}", packet.header);

    for q in packet.questions {
        println!("{:#?}", q);
    }
    for rec in packet.answers {
        println!("{:#?}", rec);
    }
    for rec in packet.authoritiees {
        println!("{:#?}", rec);
    }
    for rec in packet.resources {
        println!("{:#?}", rec);
    }

    Ok(())
}
