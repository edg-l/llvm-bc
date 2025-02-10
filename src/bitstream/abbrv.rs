use super::{
    operand::{OperandDef, OperandValue},
    BitStream,
};

const IS_LITERAL_WIDTH: u32 = 1;
const LITERAL_VALUE_WIDTH: u32 = 8;
const ENC_WIDTH: u32 = 3;
const VALUE_WIDTH: u32 = 5;
const OPERAND_COUNT_WIDTH: u32 = 5;

const LITERAL: u32 = 1;
const NOT_LITERAL: u32 = 0;

const FIXED_ENC: u32 = 1;
const VBR_ENC: u32 = 2;
const ARRAY_ENC: u32 = 3;
const CHAR6_ENC: u32 = 4;
const BLOB_ENC: u32 = 5;

#[derive(Debug, Clone)]
pub struct Abbr {
    pub name: String,
    pub operands: Vec<OperandDef>,
    pub operand_count: u32,
}

impl Abbr {
    pub fn new(name: &str, operands: &[OperandDef]) -> Self {
        let mut count = 0;

        for op in operands {
            count += op.count();
        }

        Self {
            name: name.to_string(),
            operands: operands.to_vec(),
            operand_count: count as u32,
        }
    }

    pub fn write_definition(&self, writer: &mut BitStream) {
        writer.write_vbr(self.operand_count, OPERAND_COUNT_WIDTH);

        Self::define_operands(writer, &self.operands);
    }

    pub fn define_operands(writer: &mut BitStream, operands: &[OperandDef]) {
        for op in operands {
            match op {
                OperandDef::Literal(value) => {
                    writer.writer.write_bits(LITERAL, IS_LITERAL_WIDTH);
                    writer.write_vbr(*value, LITERAL_VALUE_WIDTH);
                }
                OperandDef::Vbr(width) => {
                    writer.writer.write_bits(NOT_LITERAL, IS_LITERAL_WIDTH);
                    writer.writer.write_bits(VBR_ENC, ENC_WIDTH);
                    writer.write_vbr(*width, VALUE_WIDTH);
                }
                OperandDef::Fixed(width) => {
                    writer.writer.write_bits(NOT_LITERAL, IS_LITERAL_WIDTH);
                    writer.writer.write_bits(FIXED_ENC, ENC_WIDTH);
                    writer.write_vbr(*width, VALUE_WIDTH);
                }
                OperandDef::Array(operands) => {
                    writer.writer.write_bits(NOT_LITERAL, IS_LITERAL_WIDTH);
                    writer.writer.write_bits(ARRAY_ENC, ENC_WIDTH);
                    Self::define_operands(writer, &[*(operands).clone()]);
                }
                OperandDef::Blob => {
                    writer.writer.write_bits(NOT_LITERAL, IS_LITERAL_WIDTH);
                    writer.writer.write_bits(BLOB_ENC, ENC_WIDTH);
                }
                OperandDef::Char6 => {
                    writer.writer.write_bits(NOT_LITERAL, IS_LITERAL_WIDTH);
                    writer.writer.write_bits(CHAR6_ENC, ENC_WIDTH);
                }
            }
        }
    }

    pub fn write(&self, writer: &mut BitStream, values: &[OperandValue]) {
        // todo check matching
        for op in values {
            op.encode(writer);
        }
    }
}
