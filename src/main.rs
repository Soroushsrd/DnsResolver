use std::{io::Read, net::UdpSocket};

use dnsmsg::DnsPackets;
use header::ResultCode;
use packet::BytePacketBuffer;
use question::{DnsQuestion, QueryType};

pub mod dnsmsg;
pub mod header;
pub mod packet;
pub mod question;
pub mod record;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let socket = UdpSocket::bind(("0.0.0.0", 2053))?;
    println!("Entering the main loop...");
    //sequentially receiving queries!
    loop {
        if let Err(e) = handle_query(&socket) {
            eprint!("an error occured: {:?}", e);
        }
    }
}

fn handle_query(socket: &UdpSocket) -> Result<(), Box<dyn std::error::Error>> {
    // a buffer to read from socket onto
    let mut req_buff = BytePacketBuffer::new();

    // block until we receive a bytepacket
    let (_, source) = socket.recv_from(&mut req_buff.buff)?;

    //parsing the msg into a dns packet
    let mut request_packet = DnsPackets::from_buffer(&mut req_buff)?;

    // creating a dnspacket as a response
    let mut res_packet = DnsPackets::new();
    res_packet.header.id = request_packet.header.id;
    res_packet.header.recursion_desired = true;
    res_packet.header.recursion_available = true;
    res_packet.header.response = true;

    // cosnidering one question..
    if let Some(question) = request_packet.questions.pop() {
        println!("Received a query: {:?}", question);

        //if query fails, SERVFAIL will be returned
        //otherwise question and response records are copied into our response
        if let Ok(result) = lookup(&question.name, question.qtype) {
            res_packet.questions.push(question);
            res_packet.header.rescode = result.header.rescode;

            for rec in result.answers {
                println!("Answer: {:?}", rec);
                res_packet.answers.push(rec);
            }
            for rec in result.authoritiees {
                println!("Authority: {:?}", rec);
                res_packet.authoritiees.push(rec);
            }
            for rec in result.resources {
                println!("Resource: {:?}", rec);
                res_packet.resources.push(rec);
            }
        } else {
            res_packet.header.rescode = ResultCode::ServFail;
        }
    } else {
        // if a question is not present we return FORMERR
        // indicates that sender made a mistake
        res_packet.header.rescode = ResultCode::FormerR;
    }
    // encode and send the response
    let mut res_buffer = BytePacketBuffer::new();
    res_packet.write(&mut res_buffer)?;

    let len = res_buffer.pos();
    let data = res_buffer.get_range(0, len)?;
    socket.send_to(data, source)?;

    Ok(())
}

fn lookup(qname: &str, qtype: QueryType) -> Result<DnsPackets, Box<dyn std::error::Error>> {
    // Using googles public DNS server
    let server = ("8.8.8.8", 53);

    // Bind a UDP socket to an arbitrary port
    let socket = UdpSocket::bind(("0.0.0.0", 43210))?;

    // Build our query packet. It's important that we remember to set the
    // `recursion_desired` flag. As noted earlier, the packet id is arbitrary.
    let mut packet = DnsPackets::new();
    packet.header.id = 6666;
    packet.header.questions = 1;
    packet.header.recursion_desired = false;
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
    Ok(res_packet)
}
