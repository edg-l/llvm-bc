use std::collections::HashMap;

use abbrv::Abbr;
use bitstream_writer::BitStreamWriter;
use block::Block;
use operand::OperandValue;

mod abbrv;
mod bitstream_writer;
mod block;
mod module;
mod operand;

const ROOT_ABBR_ID_WIDTH: u32 = 2;
const BLOCK_ID_WIDTH: u32 = 8;
const NEW_ABBR_ID_WIDTH_WIDTH: u32 = 4;
const CODE_WIDTH: u32 = 6;
const NUM_OPS_WIDTH: u32 = 6;
const OP_WIDTH: u32 = 6;

const END_BLOCK: u32 = 0;
const ENTER_SUBBLOCK: u32 = 1;
const DEFINE_ABBREV: u32 = 2;
const UNABBREV_RECORD: u32 = 3;

const BLOCKINFO: u32 = 0;
const SETBID: u32 = 1;

pub struct StackElem {
    pub block: Block,
    // Offset in bytes to write the length of the block after its finished.
    pub length_offset: usize,
    // Offset in bytes at start of block contents (not header)
    pub offset: usize,
}

pub type BlockInfoMap = HashMap<u32, Vec<Abbr>>;

pub struct BitStream {
    pub(crate) writer: BitStreamWriter,
    stack: Vec<StackElem>,
    block_info: BlockInfoMap,
}

impl BitStream {
    pub fn new(magic: u32) -> Self {
        let mut s = Self {
            writer: BitStreamWriter::new(),
            stack: Default::default(),
            block_info: Default::default(),
        };

        s.writer.write_dword(magic);

        s
    }

    pub fn enter_block(&mut self, id: u32, abbr_id_width: u32) {
        self.write_abbr_id(ENTER_SUBBLOCK);
        self.write_vbr(id, BLOCK_ID_WIDTH);
        self.write_vbr(abbr_id_width, NEW_ABBR_ID_WIDTH_WIDTH);
        self.writer.align(32);

        let length_offset = self.writer.buffer.len();
        self.writer.write_dword(0); // future length value
        let offset = self.writer.buffer.len();

        let global_abbrs = self.block_info.get(&id).cloned().unwrap_or_default();

        self.stack.push(StackElem {
            block: Block::new(id, abbr_id_width, global_abbrs),
            length_offset,
            offset,
        });
    }

    pub fn end_block(&mut self, id: u32) {
        self.write_abbr_id(END_BLOCK);
        self.writer.align(32);

        assert!(!self.stack.is_empty(), "no blocks to end");

        let elem = self.stack.pop().unwrap();
        assert_eq!(elem.block.id, id, "ending invalid block");

        let computed_len: u32 = ((self.writer.buffer.len() - elem.offset) / 4) as u32;
        dbg!(&computed_len);
        let computed_len = computed_len.to_le_bytes();
        dbg!(&computed_len);

        for (i, byte) in computed_len.into_iter().enumerate() {
            self.writer.buffer[elem.length_offset + i] = byte;
        }
    }

    pub fn write_block_info(&mut self, map: &BlockInfoMap) {
        if map.is_empty() {
            return;
        }

        assert!(
            self.block_info.is_empty(),
            "block info can only be written once"
        );

        self.block_info = map.clone();

        self.enter_block(BLOCKINFO, ROOT_ABBR_ID_WIDTH);

        for (id, abbrs) in map.iter() {
            self.write_unabrr_record(SETBID, &[*id]);
            for abbr in abbrs {
                self.define_abbr(abbr);
            }
        }
        self.end_block(BLOCKINFO);
    }

    pub fn define_abbr(&mut self, abbr: &Abbr) {
        assert!(!self.stack.is_empty());

        self.write_abbr_id(DEFINE_ABBREV);
        abbr.write_definition(self);
        let block = self.stack.last_mut().unwrap();
        block.block.add_abbr(abbr.clone());
    }

    pub fn write_record(&mut self, abbr_name: &str, operands: &[OperandValue]) {
        let block = &self.stack[self.stack.len() - 1].block;
        let entry = block
            .abbr_map
            .get(abbr_name)
            .expect("no abbr found")
            .clone();

        self.write_abbr_id(entry.index);
        entry.abbr.write(self, operands);
    }

    pub fn write_unabrr_record(&mut self, code: u32, values: &[u32]) {
        self.write_abbr_id(UNABBREV_RECORD);
        self.write_vbr(code, CODE_WIDTH);
        self.write_vbr(values.len() as u32, NUM_OPS_WIDTH);
        for value in values {
            self.write_vbr(*value, OP_WIDTH);
        }
    }

    pub fn write_vbr(&mut self, mut value: u32, width: u32) {
        assert!((2..=32).contains(&width), "Invalid bit size for VBR");

        let value_bits = width - 1;
        let mask = (1 << value_bits) - 1;
        let vbr = 1 << value_bits;

        while value > mask {
            let left = value >> value_bits;
            self.writer.write_bits(vbr | (value & mask), width);
            value = left;
        }

        self.writer.write_bits(value, width);
    }

    pub fn write_vbr_u64(&mut self, value: u64, width: u32) {
        assert!((2..=64).contains(&width), "Invalid bit size for VBR");

        let mut hi = ((value >> 32) & u32::MAX as u64) as u32;
        let mut lo = (value & u32::MAX as u64) as u32;

        if hi == 0 {
            return self.write_vbr(lo, width.min(32));
        }

        let value_bits = width - 1;
        let mask = (1 << value_bits) - 1;
        let vbr = 1 << value_bits;

        while hi != 0 {
            let left = (hi & mask) << (32 - value_bits) | (lo >> value_bits);

            if left == 0 {
                break;
            }

            self.writer.write_bits(vbr | (lo & mask), width);
            lo = left;
            hi >>= value_bits;
        }

        self.writer.write_bits(lo, width);
    }

    pub fn write_abbr_id(&mut self, id: u32) {
        let len = self.stack.len();
        let width = if len == 0 {
            ROOT_ABBR_ID_WIDTH
        } else {
            self.stack[len - 1].block.abbr_id_width
        };

        self.writer.write_bits(id, width);
    }

    pub fn align(&mut self, align: u32) {
        self.writer.align(align);
    }
}

#[cfg(test)]
mod tests {
    use super::BitStream;

    #[test]
    pub fn write_vbt_32bit() {
        let mut writer = BitStream::new(0xdeadbeef);
        writer.write_vbr(0x1e, 4);
        writer.writer.align(32);
        writer.writer.flush();
        let content = hex::encode(&writer.writer.buffer[4..]);
        assert_eq!(content, "3e000000");
    }

    #[test]
    pub fn write_block() {
        let mut writer = BitStream::new(0xdeadbeef);
        writer.enter_block(50, 4);
        writer.end_block(50);
        writer.writer.flush();

        std::fs::write("out.bc", &writer.writer.buffer).unwrap();
        let content = hex::encode(&writer.writer.buffer[4..]);
        assert_eq!(content, "c91000000100000000000000");
    }
}
