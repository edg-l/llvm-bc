use crate::bitstream::BitStream;

const LEN_WIDTH: u32 = 6;

#[derive(Debug, Clone)]
pub struct FixedValue {
    pub value: u32,
    pub width: u32,
}

#[derive(Debug, Clone)]
pub struct VBRValue {
    pub value: u64,
    pub width: u32,
}

#[derive(Debug, Clone)]
pub enum OperandDef {
    Literal(u32),
    Vbr(u32),
    Fixed(u32),
    Array(Box<Self>),
    Blob,
    Char6,
}

#[derive(Debug, Clone)]
pub enum OperandValue {
    Literal,
    Vbr(VBRValue),
    Fixed(FixedValue),
    Array(Vec<Self>),
    Blob(Vec<u8>),
    Char6(char),
}

impl From<&str> for OperandValue {
    fn from(value: &str) -> Self {
        assert!(value.is_ascii());

        let mut arr = vec![];

        for char in value.chars() {
            arr.push(Self::Char6(char));
        }

        OperandValue::Array(arr)
    }
}

impl OperandValue {
    pub fn encode(&self, stream: &mut BitStream) {
        match self {
            OperandValue::Literal => {}
            OperandValue::Vbr(vbrvalue) => {
                stream.write_vbr_u64(
                    (vbrvalue.value >> 32) as u32,
                    vbrvalue.value as u32,
                    vbrvalue.width,
                );
            }
            OperandValue::Fixed(fixed_value) => {
                stream
                    .writer
                    .write_bits(fixed_value.value, fixed_value.width);
            }
            OperandValue::Array(operands) => {
                stream.write_vbr(operands.len() as u32, LEN_WIDTH);

                for op in operands {
                    op.encode(stream);
                }
            }
            OperandValue::Blob(blob_value) => {
                stream.write_vbr(blob_value.len() as u32, LEN_WIDTH);
                stream.align(32);

                for bits in blob_value {
                    stream.writer.write_bits(*bits as u32, 8);
                }
                stream.align(32);
            }
            OperandValue::Char6(code) => {
                let mut code: u32 = (*code) as u32;
                // 'a' - 'z'
                if (0x61..=0x7a).contains(&code) {
                    code -= 0x61;
                }
                // 'A' - 'Z'
                else if (0x41..=0x5a).contains(&code) {
                    code = code - 0x41 + 26;
                // '0' - '9'
                } else if (0x30..=0x39).contains(&code) {
                    code = code - 0x30 + 52;
                // '.'
                } else if code == 0x2e {
                    code = 62;
                // '_'
                } else if code == 0x5f {
                    code = 63;
                } else {
                    panic!("invalid char6")
                }
                stream.writer.write_bits(code, 6);
            }
        }
    }
}

impl OperandDef {
    pub fn count(&self) -> usize {
        match self {
            OperandDef::Literal(_) => 1,
            OperandDef::Vbr(_vbrvalue) => 1,
            OperandDef::Fixed(_fixed_value) => 1,
            OperandDef::Array(operand) => 1 + operand.count(),
            OperandDef::Blob => 1,
            OperandDef::Char6 => 1,
        }
    }
}
