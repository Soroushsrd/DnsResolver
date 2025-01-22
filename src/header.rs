use crate::packet::BytePacketBuffer;

/// for reference purposes:
///
/// DNS Header
//                                    1  1  1  1  1  1
//  0  1  2  3  4  5  6  7  8  9  0  1  2  3  4  5
//+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
//|                      ID                         |
//+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
//|QR|   Opcode  |AA|TC|RD|RA|   Z    |   RCODE   |
//+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
//////////////////////
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ResultCode {
    NoError = 0,
    FormerR = 1,
    ServFail = 2,
    NXDomain = 3,
    NOTimP = 4,
    Refused = 5,
}

impl From<u8> for ResultCode {
    fn from(value: u8) -> Self {
        match value {
            1 => ResultCode::FormerR,
            2 => ResultCode::ServFail,
            3 => ResultCode::NXDomain,
            4 => ResultCode::NOTimP,
            5 => ResultCode::Refused,
            0 | _ => ResultCode::NoError,
        }
    }
}

// mind the types, eg: u16 => 16 bits
#[derive(Debug, Clone)]
pub struct DnsHeader {
    pub id: u16,

    pub recursion_desired: bool, // 1bit
    pub truncated_msg: bool,
    pub authorative_answer: bool,
    pub opcode: u8, //4 bits
    pub response: bool,

    pub rescode: ResultCode, //4bits
    pub checking_disabled: bool,
    pub authed_data: bool,
    pub z: bool,
    pub recursion_available: bool,

    pub questions: u16,
    pub answers: u16,
    pub authorative_entries: u16,
    pub resource_entries: u16,
}

impl DnsHeader {
    pub fn new() -> Self {
        Self {
            id: 0,

            recursion_desired: false,
            truncated_msg: false,
            authorative_answer: false,
            opcode: 0,
            response: false,

            rescode: ResultCode::NoError,
            checking_disabled: false,
            authed_data: false,
            z: false,
            recursion_available: false,

            questions: 0,
            answers: 0,
            authorative_entries: 0,
            resource_entries: 0,
        }
    }
    pub fn write(
        &mut self,
        packet: &mut BytePacketBuffer,
    ) -> Result<(), Box<dyn std::error::Error>> {
        packet.write_u16(self.id)?;
        packet.write(
            (self.recursion_desired as u8)
                | ((self.truncated_msg as u8) << 1)
                | ((self.authorative_answer as u8) << 2)
                | ((self.opcode as u8) << 3)
                | ((self.response as u8) << 7),
        )?;
        packet.write(
            (self.rescode as u8)
                | ((self.checking_disabled as u8) << 4)
                | ((self.authed_data as u8) << 5)
                | ((self.z as u8) << 6)
                | ((self.recursion_available as u8) << 7),
        )?;
        packet.write_u16(self.questions)?;
        packet.write_u16(self.answers)?;
        packet.write_u16(self.authorative_entries)?;
        packet.write_u16(self.resource_entries)?;
        Ok(())
    }

    pub fn read(
        &mut self,
        packet: &mut BytePacketBuffer,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.id = packet.read_u16().expect("failed to read id");

        let flags = packet.read_u16().expect("failed to read flags");

        let a = (flags >> 8) as u8; //getting the first 8bits
        let b = (flags & 0xFF) as u8; //getting the second 8bits with a mask- masks 8 bits?!

        //For single flag bits (QR, AA, TC, etc), we use & (1 << n) where n is the bit position
        self.recursion_desired = (a & (1 << 0)) > 0;
        self.truncated_msg = (a & (1 << 1)) > 0;
        self.authorative_answer = (a & (1 << 2)) > 0;
        self.opcode = (a >> 3) & 0x0F; // shifts forward 3 bits (jumping over the previous ones) and reads the next 4 beats using a mask
        self.response = (a & (1 << 7)) > 0;

        // Gets bits 0-3 --> last bits though
        self.rescode = ResultCode::from(b & 0x0F);

        self.checking_disabled = (b & (1 << 4)) > 0; //gets bit 4
        self.authed_data = (b & (1 << 5)) > 0;
        self.z = (b & (1 << 6)) > 0;
        self.recursion_available = (b & (1 << 7)) > 0;

        self.questions = packet.read_u16()?;
        self.answers = packet.read_u16()?;
        self.authorative_entries = packet.read_u16()?;
        self.resource_entries = packet.read_u16()?;

        Ok(())
    }
}
