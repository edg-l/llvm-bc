use std::io::Write;

use bytes::{Buf, BufMut, Bytes};

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
        self.write_bits(dword, 32);
    }

    pub fn write_word(&mut self, word: u16) {
        self.write_bits(word as u32, 16);
    }

    pub fn write_byte(&mut self, byte: u8) {
        self.write_bits(byte as u32, 8);
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
            self.buffer.put_u32_le(self.dword);
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
            self.buffer.put_u16_le((self.dword >> 8) as u16);
        } else if self.dword_offset == 16 {
            self.buffer.put_u16_le(self.dword as u16)
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
    use std::io::Read;

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

    #[test]
    fn write_words_and_bytes() {
        let mut writer = BitStreamWriter::new();

        let mut i = 0;
        while i < 30_000 {
            writer.write_word(i as u16);
            writer.write_byte(i as u8);
            i += 3;
        }

        assert_eq!(writer.bit_offset(), 30_000 * 8);
        writer.flush();
        assert_eq!(writer.bit_offset(), 30_000 * 8);

        assert_eq!(writer.buffer.len(), 30_000);

        let mut i = 0;
        let mut buf = writer.buffer.as_slice();
        while i < 30_000 {
            assert_eq!(buf.get_u16_le(), i as u16, "Mismatch at {i}");
            assert_eq!(buf.get_u8(), i as u8, "Mismatch at {i}");
            i += 3;
        }
    }

    #[test]
    fn write_dwords_and_bytes() {
        let mut writer = BitStreamWriter::new();

        let mut i = 0;
        while i < 30_000 {
            writer.write_dword(i);
            writer.write_byte(i as u8);
            i += 5;
        }

        assert_eq!(writer.bit_offset(), 30_000 * 8);
        writer.flush();
        assert_eq!(writer.bit_offset(), 30_000 * 8);

        assert_eq!(writer.buffer.len(), 30_000);

        let mut i = 0;
        let mut buf = writer.buffer.as_slice();
        while i < 30_000 {
            assert_eq!(buf.get_u32_le(), i as u32, "Mismatch at {i}");
            assert_eq!(buf.get_u8(), i as u8, "Mismatch at {i}");
            i += 5;
        }
    }
}
