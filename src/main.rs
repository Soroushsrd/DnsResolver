use std::{fs::File, io::Read, net::UdpSocket};

use dnsmsg::DnsPackets;
use packet::BytePacketBuffer;
use question::{DnsQuestion, QueryType};

pub mod dnsmsg;
pub mod header;
pub mod packet;
pub mod question;
pub mod record;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Perform an A query for google.com
    let qname = "google.com";
    let qtype = QueryType::A;

    // Using googles public DNS server
    let server = ("8.8.8.8", 53);

    // Bind a UDP socket to an arbitrary port
    let socket = UdpSocket::bind(("0.0.0.0", 43210))?;

    // Build our query packet. It's important that we remember to set the
    // `recursion_desired` flag. As noted earlier, the packet id is arbitrary.
    let mut packet = DnsPackets::new();
    packet.header.id = 6666;
    packet.header.questions = 1;
    packet.header.recursion_desired = true;
    packet
        .questions
        .push(DnsQuestion::new(qname.to_string(), qtype));

    // now we create a buffer to write to!
    let mut req_buff = BytePacketBuffer::new();
    packet.write(&mut req_buff)?;

    socket.send_to(&req_buff.buff[0..req_buff.pos], server)?;

    // creating a receiving buff
    let mut res_buff = BytePacketBuffer::new();
    socket.recv_from(&mut res_buff.buff)?;

    //now the parsing part:
    let res_packet = DnsPackets::from_buffer(&mut res_buff)?;

    println!("{:#?}", res_packet.header);
    for q in res_packet.questions {
        println!("{:#?}", q);
    }
    for rec in res_packet.answers {
        println!("{:#?}", rec);
    }
    for rec in res_packet.authoritiees {
        println!("{:#?}", rec);
    }
    for rec in res_packet.resources {
        println!("{:#?}", rec);
    }

    Ok(())
}
