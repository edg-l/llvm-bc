// https://github.com/llvm/llvm-project/blob/llvmorg-19.1.7/llvm/include/llvm/Bitcode/LLVMBitCodes.h
// https://github.com/llvm/llvm-project/blob/llvmorg-19.1.7/llvm/lib/Bitcode/Writer/BitcodeWriter.cpp

// https://llvm.org/docs/BitCodeFormat.html#llvm-ir-blocks
#[derive(Debug, Clone, Copy)]
pub enum BlockId {
    Module = 8,
    ParamAttr = 9,
    ParamAttrGroup = 10,
    Constants = 11,
    Function = 12,
    Identification = 13,
    ValueSymtab = 14,
    Metadata = 15,
    MetadataAttachment = 16,
    Type = 17,
    UseList = 18,
    ModuleStrab = 19,
    GlobalValSummary = 20,
    OperandBundleTags = 21,
    MetadataKind = 22,
    Strtab = 23,
    FullLtoGlobalValSummary = 24,
    Symtab = 25,
    SyncScopeNames = 26,
}

#[derive(Debug, Clone, Copy)]
pub enum IdentificationCodes {
    String = 1,
    Epoch = 2,
}

const CurrentEpoch: u32 = 0;

// https://llvm.org/docs/BitCodeFormat.html#module-block-contents
// https://github.com/llvm/llvm-project/blob/cd708029e0b2869e80abe31ddb175f7c35361f90/llvm/include/llvm/Bitcode/LLVMBitCodes.h#L84
#[derive(Debug, Clone, Copy)]
pub enum ModuleCode {
    Version = 1,
    Triple = 2,
    DataLayout = 3,
    Asm = 4,
    SectionName = 5,
    Deplib = 6,
    GlobalVar = 7,
    Function = 8,
    AliasOld = 9,
    GcName = 11,
    Comdat = 12,
    VstOffset = 13,
    Alias = 14,
    MetadataValuesUnused = 15,
    SourceFilename = 16,
    CodeHash = 17,
    CodeIfunc = 18,
}

#[derive(Debug, Clone, Copy)]
pub enum AttributeCode {
    EntryOld = 1,
    CodeEntry = 2,
    GrpCodeEntry = 3,
}

// https://llvm.org/docs/BitCodeFormat.html#type-block-contents
#[derive(Debug, Clone, Copy)]
pub enum TypeCode {
    Numentry = 1,
    Void = 2,
    Float = 3,
    Double = 4,
    Label = 5,
    Opaque = 6,
    Integer = 7,
    Pointer = 8,
    FunctionOld = 9,
    Half = 10,
    Array = 11,
    Vector = 12,
    X86Fp80 = 13,
    Fp128 = 14,
    PpcFp128 = 15,
    Metadata = 16,
    X86Mmx = 17,
    StructAnon = 18,
    StructName = 19,
    StructNamed = 20,
    Function = 21,
    Bfloat = 23,
    X86Amx = 24,
    OpaquePointer = 25,
    TargetType = 26,
}

#[derive(Debug, Clone, Copy)]
pub enum OperandBundleTagCode {
    BundleTag = 1,
}

#[derive(Debug, Clone, Copy)]
pub enum SyncScopeNameCode {
    Name = 1,
}

#[derive(Debug, Clone, Copy)]
pub enum ValueSymtabCode {
    Entry = 1,
    BbEntry = 2,
    FnEntry = 3,
    CombinedEntry = 5,
}

#[derive(Debug, Clone, Copy)]
pub enum ModulePathSymtabCode {
    Entry = 1,
    Hash = 2,
}

#[derive(Debug, Clone, Copy)]
pub enum StrtabCode {
    Blob = 1,
}

#[derive(Debug, Clone, Copy)]
pub enum ConstantsCode {
    SetType = 1,
    Null = 2,
    Undef = 3,
    Integer = 4,
    WideInteger = 5,
    Float = 6,
    Aggregate = 7,
    String = 8,
    Cstring = 9,
    CeBinop = 10,
    CeCast = 11,
    CeGepOld = 12,
    CeSelect = 13,
    CeExtractElt = 14,
    CeInsertElt = 15,
    CeShuffleVec = 16,
    CeCmp = 17,
    CeInlineAsmOld = 18,

    CeShufVecEx = 19,
    CeInboundsGep = 20,

    BlockAddress = 21,
    Data = 22,
    InlineAsmOld2 = 23,

    CeGepWithInRangeIndexOld = 24,
    CeUnOp = 25,
    Poison = 26,
    DsoLocalEquivalent = 27,
    InlineAsmOld3 = 28,

    CodeNoCfiValue = 29,
    InlineAsm = 30,
    CeGepWithInrange = 31,
    CeGep = 32,
    PtrAuth = 33,
}

#[derive(Debug, Clone, Copy)]
pub enum CastCodes {
    Trunc = 0,
    Zext = 1,
    Sext = 2,
    FpToUi = 3,
    FpToSi = 4,
    UiToFp = 5,
    SiToFp = 6,
    FpTrunc = 7,
    FpExt = 8,
    PtrToInt = 9,
    IntToPtr = 10,
    Bitcast = 11,
    AddspaceCast = 12,
}

#[derive(Debug, Clone, Copy)]
pub enum UnaryOpCodes {
    Fneg = 0,
}

#[derive(Debug, Clone, Copy)]
pub enum BinaryOpCode {
    Add = 0,
    Sub = 1,
    Mul = 2,
    UDiv = 3,
    SDiv = 4,
    URem = 5,
    SRem = 6,
    Shl = 7,
    LShr = 8,
    AShr = 9,
    And = 10,
    Or = 11,
    Xor = 12,
}

#[derive(Debug, Clone, Copy)]
pub enum RMWOperation {
    XCHG = 0,
    ADD = 1,
    SUB = 2,
    AND = 3,
    NAND = 4,
    OR = 5,
    XOR = 6,
    MAX = 7,
    MIN = 8,
    UMAX = 9,
    UMIN = 10,
    FADD = 11,
    FSUB = 12,
    FMAX = 13,
    FMIN = 14,
    UincWrap = 15,
    UdecWrap = 16,
}
