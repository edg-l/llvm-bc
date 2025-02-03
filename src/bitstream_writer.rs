use bytes::BufMut;

#[derive(Debug, Clone, Default)]
pub struct BitStreamWriter {
    pub(crate) buffer: Vec<u8>,
    // current dword,
    dword: u32,

    // bits left in current dword
    dword_left: u32,

    dword_offset: u32,
}

impl BitStreamWriter {
    pub fn new() -> Self {
        Self {
            buffer: Vec::with_capacity(1024),
            dword: 0,
            dword_left: 32,
            dword_offset: 0,
        }
    }

    pub fn write_dword(&mut self, dword: u32) {
        self.buffer.put_u32(dword);
    }

    pub fn write_bits(&mut self, value: u32, width: u32) {
        if width == 0 {
            return;
        }

        assert!(0 < width && width <= 32, "Invalid number of bits");

        let fits = self.dword_left.min(width);

        let mask: u32 = if fits == 32 {
            0xffffffff
        } else {
            (1 << fits) - 1
        };

        if fits < width {
            self.write_bits(value & mask, fits);
            self.write_bits(value >> fits, width - fits);
            return;
        }

        self.dword |= (value & mask) << self.dword_offset;
        self.dword_offset += width;
        self.dword_left -= width;

        if self.dword_left == 0 {
            self.buffer.put_u32(self.dword);
            self.dword = 0;
            self.dword_offset = 0;
            self.dword_left = 32;
        }
    }

    pub fn bit_offset(&self) -> usize {
        self.buffer.len() * 8 + self.dword_offset as usize
    }

    pub fn pad(&mut self, width: u32) {
        self.write_bits(0, width);
    }

    pub fn align(&mut self, width: u32) {
        self.pad(self.dword_left % width);
    }

    pub fn flush(&mut self) {
        if self.dword_offset == 0 {
            return;
        }

        self.align(8);

        if self.dword_offset == 8 + 16 {
            self.buffer.put_u8((self.dword & 0xff) as u8);
            self.buffer.put_u16((self.dword >> 8) as u16);
        } else if self.dword_offset == 16 {
            self.buffer.put_u16(self.dword as u16)
        } else if self.dword_offset == 8 {
            self.buffer.put_u8(self.dword as u8);
        } else {
            assert_eq!(self.dword_offset, 0);
        }

        self.dword_left = 32;
        self.dword_offset = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_bits() {
        let mut writer = BitStreamWriter::new();

        writer.write_bits(3, 31);
        assert_eq!(writer.bit_offset(), 31);
        assert_eq!(writer.dword_left, 1);
        assert_eq!(writer.dword_offset, 31);

        writer.write_bits(7, 4);
        assert_eq!(writer.bit_offset(), 35);
        assert_eq!(writer.dword_left, 29);
        assert_eq!(writer.dword_offset, 3);

        writer.flush();
        assert_eq!(writer.bit_offset(), 40);
        assert_eq!(writer.dword_left, 32);
        assert_eq!(writer.dword_offset, 0);
    }

    #[test]
    fn aligned() {
        let mut writer = BitStreamWriter::new();
        writer.write_bits(3, 31);
        writer.align(32);
        assert_eq!(writer.bit_offset(), 32);
        writer.flush();
        assert_eq!(writer.bit_offset(), 32);
    }
}
