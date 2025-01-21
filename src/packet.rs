use core::error;

pub struct BytePacketBuffer {
    pub buff: [u8; 512],
    pub pos: usize,
}

impl BytePacketBuffer {
    pub fn new() -> Self {
        Self {
            buff: [0u8; 512],
            pos: 0,
        }
    }
    ///get the position
    pub fn pos(&self) -> usize {
        self.pos
    }
    /// Step forward
    pub fn step(&mut self, step: usize) {
        self.pos += step;
    }
    /// change the buffer positon
    pub fn seek(&mut self, pos: usize) {
        self.pos = pos;
    }
    /// read a byte and move forward the position for one step
    pub fn read(&mut self) -> Result<u8, Box<dyn std::error::Error>> {
        if self.pos >= 512 {
            return Err("End of buffer bounds".into());
        }
        let byte_read = self.buff[self.pos];
        self.pos += 1;
        Ok(byte_read)
    }
    /// get a single byte without changing the buffer position
    pub fn get(&mut self, pos: usize) -> Result<u8, Box<dyn std::error::Error>> {
        if pos >= 512 {
            return Err("Position out of bounds".into());
        }
        Ok(self.buff[pos])
    }
    /// get a range of bytes
    pub fn get_range(
        &mut self,
        start: usize,
        length: usize,
    ) -> Result<&[u8], Box<dyn std::error::Error>> {
        if start + length >= 512 {
            return Err("Out of bounds!".into());
        }
        let bytes = &self.buff[start..start + length];
        Ok(bytes)
    }

    ///read 2 bytes, stepping 2 steps forward
    pub fn read_u16(&mut self) -> Result<u16, Box<dyn std::error::Error>> {
        let ret = (self.read()? as u16) << 8 | self.read()? as u16;

        Ok(ret)
    }
    /// read four bytes, step four bytes forward
    pub fn read_u32(&mut self) -> Result<u32, Box<dyn std::error::Error>> {
        let ret = (self.read()? as u32) << 24
            | (self.read()? as u32) << 16
            | (self.read()? as u32) << 8
            | (self.read()? as u32) << 0;
        Ok(ret)
    }
    /// read a qname
    pub fn read_qname(&mut self, outstr: &mut String) -> Result<(), Box<dyn std::error::Error>> {
        let mut pos = self.pos();

        let mut jumped = false;
        let max_jumps = 5;
        let mut jumps_performed = 0;

        let mut delimiter = "";

        loop {
            if jumps_performed > max_jumps {
                return Err("Limit of jumps reached!".into());
            }

            let len = self.get(pos)?;

            if (len & 0xC0) == 0xC0 {
                if !jumped {
                    self.seek(pos + 2);
                }
                let b2 = self.get(pos + 1)? as u16;
                let offset = (((len as u16) ^ 0xC0) << 8) | b2;
                pos = offset as usize;

                // Indicate that a jump was performed.
                jumped = true;
                jumps_performed += 1;

                continue;
            } else {
                pos += 1;

                if len == 0 {
                    break;
                }

                outstr.push_str(delimiter);

                let str_out = self.get_range(pos, len as usize)?;

                outstr.push_str(&String::from_utf8_lossy(str_out).to_lowercase());

                delimiter = ".";
                pos += len as usize;
            }
        }

        if !jumped {
            self.seek(pos);
        }
        Ok(())
    }
}
