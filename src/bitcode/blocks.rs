use crate::bitstream::{abbrv::Abbr, operand::OperandDef, BlockInfoMap};

use super::constants::*;



pub struct ConstantBlock {

}

impl ConstantBlock {
    // https://github.com/llvm/llvm-project/blob/cd708029e0b2869e80abe31ddb175f7c35361f90/llvm/lib/Bitcode/Writer/BitcodeWriter.cpp#L3714
    pub fn build_info(info: &mut BlockInfoMap) {
        info.insert(BlockId::Constants as u32, vec![
            Abbr::new("settype", &[
                OperandDef::Literal(ConstantsCode::SetType as u32),
                OperandDef::Vbr(vbr_widths::TYPE_INDEX), // not sure its correct
            ]),
            Abbr::new("int", &[
                OperandDef::Literal(ConstantsCode::Integer as u32),
                OperandDef::Vbr(vbr_widths::INTEGER),
            ]),
            Abbr::new("null", &[
                OperandDef::Literal(ConstantsCode::Null as u32),
            ]),
            Abbr::new("undef", &[
                OperandDef::Literal(ConstantsCode::Undef as u32),
            ]),
            Abbr::new("aggr", &[
                OperandDef::Literal(ConstantsCode::Aggregate as u32),
                OperandDef::Array(Box::new(OperandDef::Vbr(vbr_widths::VALUE_INDEX))),
            ]),
        ]);
    }
}
