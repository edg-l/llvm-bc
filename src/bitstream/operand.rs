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
    VBR(u32),
    Fixed(u32),
    Array(Vec<Self>),
    Blob,
    Char6,
}

#[derive(Debug, Clone)]
pub enum OperandValue {
    Literal,
    VBR(VBRValue),
    Fixed(FixedValue),
    Array(Vec<Self>),
    Blob(Vec<u8>),
    Char6(u32),
}

impl OperandValue {
    pub fn encode(&self, stream: &mut BitStream) {
        match self {
            OperandValue::Literal => {}
            OperandValue::VBR(vbrvalue) => {
                stream.write_vbr_u64(vbrvalue.value, vbrvalue.width);
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
            },
            OperandValue::Blob(blob_value) => {
                stream.write_vbr(blob_value.len() as u32, LEN_WIDTH);
                stream.align(32);

                for bits in blob_value {
                    stream.writer.write_bits(*bits as u32, 8);
                }
                stream.align(32);
            }
            OperandValue::Char6(mut code) => {
                // 'a' - 'z'
                if 0x61 <= code && code <= 0x7a {
                    code = code - 0x61;
                }
                // 'A' - 'Z'
                else if 0x41 <= code && code <= 0x5a {
                    code = code - 0x41 + 26;
                // '0' - '9'
                } else if 0x30 <= code && code <= 0x39 {
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
            OperandDef::VBR(_vbrvalue) => 1,
            OperandDef::Fixed(_fixed_value) => 1,
            OperandDef::Array(operands) => {
                let mut total = 0;

                for op in operands {
                    total += op.count();
                }

                total
            }
            OperandDef::Blob => 1,
            OperandDef::Char6 => 1,
        }
    }
}
