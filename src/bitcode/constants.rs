#![allow(non_camel_case_types)] // todo: fix later

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

pub const CURRENT_EPOCH: u32 = 0;

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

#[derive(Debug, Clone, Copy)]
pub enum GetElementPtrOptionalFlag {
    GEP_INBOUNDS = 0,
    GEP_NUSW = 1,
    GEP_NUW = 2,
}

#[derive(Debug, Clone, Copy)]
pub enum AtomicOrderingCode {
    ORDERING_NOTATOMIC = 0,
    ORDERING_UNORDERED = 1,
    ORDERING_MONOTONIC = 2,
    ORDERING_ACQUIRE = 3,
    ORDERING_RELEASE = 4,
    ORDERING_ACQREL = 5,
    ORDERING_SEQCST = 6,
}

#[derive(Debug, Clone, Copy)]
pub enum CallMarkersFlag {
    CALL_TAIL = 0,
    CALL_CCONV = 1,
    CALL_MUSTTAIL = 14,
    CALL_EXPLICIT_TYPE = 15,
    CALL_NOTAIL = 16,
    CALL_FMF = 17, // Call has optional fast-math-flags.
}

#[derive(Debug, Clone, Copy)]
pub enum FunctionCide {
    DECLAREBLOCKS = 1, // DECLAREBLOCKS: [n]

    INST_BINOP = 2,      // BINOP:      [opcode, ty, opval, opval]
    INST_CAST = 3,       // CAST:       [opcode, ty, opty, opval]
    INST_GEP_OLD = 4,    // GEP:        [n x operands]
    INST_SELECT = 5,     // SELECT:     [ty, opval, opval, opval]
    INST_EXTRACTELT = 6, // EXTRACTELT: [opty, opval, opval]
    INST_INSERTELT = 7,  // INSERTELT:  [ty, opval, opval, opval]
    INST_SHUFFLEVEC = 8, // SHUFFLEVEC: [ty, opval, opval, opval]
    INST_CMP = 9,        // CMP:        [opty, opval, opval, pred]

    INST_RET = 10,    // RET:        [opty,opval<both optional>]
    INST_BR = 11,     // BR:         [bb#, bb#, cond] or [bb#]
    INST_SWITCH = 12, // SWITCH:     [opty, op0, op1, ...]
    INST_INVOKE = 13, // INVOKE:     [attr, fnty, op0,op1, ...]
    // 14 is unused.
    INST_UNREACHABLE = 15, // UNREACHABLE

    INST_PHI = 16, // PHI:        [ty, val0,bb0, ...]
    // 17 is unused.
    // 18 is unused.
    INST_ALLOCA = 19, // ALLOCA:     [instty, opty, op, align]
    INST_LOAD = 20,   // LOAD:       [opty, op, align, vol]
    // 21 is unused.
    // 22 is unused.
    INST_VAARG = 23, // VAARG:      [valistty, valist, instty]
    // This store code encodes the pointer type, rather than the value type
    // this is so information only available in the pointer type (e.g. address
    // spaces) is retained.
    INST_STORE_OLD = 24, // STORE:      [ptrty,ptr,val, align, vol]
    // 25 is unused.
    INST_EXTRACTVAL = 26, // EXTRACTVAL: [n x operands]
    INST_INSERTVAL = 27,  // INSERTVAL:  [n x operands]
    // fcmp/icmp returning Int1TY or vector of Int1Ty. Same as CMP, exists to
    // support legacy vicmp/vfcmp instructions.
    INST_CMP2 = 28, // CMP2:       [opty, opval, opval, pred]
    // new select on i1 or [N x i1]
    INST_VSELECT = 29,          // VSELECT:    [ty,opval,opval,predty,pred]
    INST_INBOUNDS_GEP_OLD = 30, // INBOUNDS_GEP: [n x operands]
    INST_INDIRECTBR = 31,       // INDIRECTBR: [opty, op0, op1, ...]
    // 32 is unused.
    DEBUG_LOC_AGAIN = 33, // DEBUG_LOC_AGAIN

    INST_CALL = 34, // CALL:    [attr, cc, fnty, fnid, args...]

    DEBUG_LOC = 35,        // DEBUG_LOC:  [Line,Col,ScopeVal, IAVal]
    INST_FENCE = 36,       // FENCE: [ordering, synchscope]
    INST_CMPXCHG_OLD = 37, // CMPXCHG: [ptrty, ptr, cmp, val, vol,
    //            ordering, synchscope,
    //            failure_ordering?, weak?]
    INST_ATOMICRMW_OLD = 38, // ATOMICRMW: [ptrty,ptr,val, operation,
    //             align, vol,
    //             ordering, synchscope]
    INST_RESUME = 39,         // RESUME:     [opval]
    INST_LANDINGPAD_OLD = 40, // LANDINGPAD: [ty,val,val,num,id0,val0...]
    INST_LOADATOMIC = 41,     // LOAD: [opty, op, align, vol,
    //        ordering, synchscope]
    INST_STOREATOMIC_OLD = 42, // STORE: [ptrty,ptr,val, align, vol
    //         ordering, synchscope]
    INST_GEP = 43,         // GEP:  [inbounds, n x operands]
    INST_STORE = 44,       // STORE: [ptrty,ptr,valty,val, align, vol]
    INST_STOREATOMIC = 45, // STORE: [ptrty,ptr,val, align, vol
    INST_CMPXCHG = 46,     // CMPXCHG: [ptrty, ptr, cmp, val, vol,
    //           success_ordering, synchscope,
    //           failure_ordering, weak]
    INST_LANDINGPAD = 47,  // LANDINGPAD: [ty,val,num,id0,val0...]
    INST_CLEANUPRET = 48,  // CLEANUPRET: [val] or [val,bb#]
    INST_CATCHRET = 49,    // CATCHRET: [val,bb#]
    INST_CATCHPAD = 50,    // CATCHPAD: [bb#,bb#,num,args...]
    INST_CLEANUPPAD = 51,  // CLEANUPPAD: [num,args...]
    INST_CATCHSWITCH = 52, // CATCHSWITCH: [num,args...] or [num,args...,bb]
    // 53 is unused.
    // 54 is unused.
    OPERAND_BUNDLE = 55, // OPERAND_BUNDLE: [tag#, value...]
    INST_UNOP = 56,      // UNOP:       [opcode, ty, opval]
    INST_CALLBR = 57,    // CALLBR:     [attr, cc, norm, transfs,
    //              fnty, fnid, args...]
    INST_FREEZE = 58,    // FREEZE: [opty, opval]
    INST_ATOMICRMW = 59, // ATOMICRMW: [ptrty, ptr, valty, val,
    //             operation, align, vol,
    //             ordering, synchscope]
    BLOCKADDR_USERS = 60, // BLOCKADDR_USERS: [value...]

    DEBUG_RECORD_VALUE = 61, // [DILocation, DILocalVariable, DIExpression, ValueAsMetadata]
    DEBUG_RECORD_DECLARE = 62, // [DILocation, DILocalVariable, DIExpression, ValueAsMetadata]
    DEBUG_RECORD_ASSIGN = 63, // [DILocation, DILocalVariable, DIExpression, ValueAsMetadata,
    //  DIAssignID, DIExpression (addr), ValueAsMetadata (addr)]
    DEBUG_RECORD_VALUE_SIMPLE = 64, // [DILocation, DILocalVariable, DIExpression, Value]
    DEBUG_RECORD_LABEL = 65,        // [DILocation, DILabel]
}

#[derive(Debug, Clone, Copy)]
pub enum UseListCode {
    CODE_DEFAULT = 1, // DEFAULT: [index..., value-id]
    CODE_BB = 2,      // BB: [index..., bb-id]
}

#[derive(Debug, Clone, Copy)]
pub enum AttributeKindCode {
    // = 0 is unused
    ALIGNMENT = 1,
    ALWAYS_INLINE = 2,
    BY_VAL = 3,
    INLINE_HINT = 4,
    IN_REG = 5,
    MIN_SIZE = 6,
    NAKED = 7,
    NEST = 8,
    NO_ALIAS = 9,
    NO_BUILTIN = 10,
    NO_CAPTURE = 11,
    NO_DUPLICATE = 12,
    NO_IMPLICIT_FLOAT = 13,
    NO_INLINE = 14,
    NON_LAZY_BIND = 15,
    NO_RED_ZONE = 16,
    NO_RETURN = 17,
    NO_UNWIND = 18,
    OPTIMIZE_FOR_SIZE = 19,
    READ_NONE = 20,
    READ_ONLY = 21,
    RETURNED = 22,
    RETURNS_TWICE = 23,
    S_EXT = 24,
    STACK_ALIGNMENT = 25,
    STACK_PROTECT = 26,
    STACK_PROTECT_REQ = 27,
    STACK_PROTECT_STRONG = 28,
    STRUCT_RET = 29,
    SANITIZE_ADDRESS = 30,
    SANITIZE_THREAD = 31,
    SANITIZE_MEMORY = 32,
    UW_TABLE = 33,
    Z_EXT = 34,
    BUILTIN = 35,
    COLD = 36,
    OPTIMIZE_NONE = 37,
    IN_ALLOCA = 38,
    NON_NULL = 39,
    JUMP_TABLE = 40,
    DEREFERENCEABLE = 41,
    DEREFERENCEABLE_OR_NULL = 42,
    CONVERGENT = 43,
    SAFESTACK = 44,
    ARGMEMONLY = 45,
    SWIFT_SELF = 46,
    SWIFT_ERROR = 47,
    NO_RECURSE = 48,
    INACCESSIBLEMEM_ONLY = 49,
    INACCESSIBLEMEM_OR_ARGMEMONLY = 50,
    ALLOC_SIZE = 51,
    WRITEONLY = 52,
    SPECULATABLE = 53,
    STRICT_FP = 54,
    SANITIZE_HWADDRESS = 55,
    NOCF_CHECK = 56,
    OPT_FOR_FUZZING = 57,
    SHADOWCALLSTACK = 58,
    SPECULATIVE_LOAD_HARDENING = 59,
    IMMARG = 60,
    WILLRETURN = 61,
    NOFREE = 62,
    NOSYNC = 63,
    SANITIZE_MEMTAG = 64,
    PREALLOCATED = 65,
    NO_MERGE = 66,
    NULL_POINTER_IS_VALID = 67,
    NOUNDEF = 68,
    BYREF = 69,
    MUSTPROGRESS = 70,
    NO_CALLBACK = 71,
    HOT = 72,
    NO_PROFILE = 73,
    VSCALE_RANGE = 74,
    SWIFT_ASYNC = 75,
    NO_SANITIZE_COVERAGE = 76,
    ELEMENTTYPE = 77,
    DISABLE_SANITIZER_INSTRUMENTATION = 78,
    NO_SANITIZE_BOUNDS = 79,
    ALLOC_ALIGN = 80,
    ALLOCATED_POINTER = 81,
    ALLOC_KIND = 82,
    PRESPLIT_COROUTINE = 83,
    FNRETTHUNK_EXTERN = 84,
    SKIP_PROFILE = 85,
    MEMORY = 86,
    NOFPCLASS = 87,
    OPTIMIZE_FOR_DEBUGGING = 88,
    WRITABLE = 89,
    CORO_ONLY_DESTROY_WHEN_COMPLETE = 90,
    DEAD_ON_UNWIND = 91,
    RANGE = 92,
    SANITIZE_NUMERICAL_STABILITY = 93,
    INITIALIZES = 94,
    HYBRID_PATCHABLE = 95,
}

#[derive(Debug, Clone, Copy)]
pub enum ComdatSelectionKindCode {
    ANY = 1,
    EXACT_MATCH = 2,
    LARGEST = 3,
    NO_DUPLICATES = 4,
    SAME_SIZE = 5,
}

#[derive(Debug, Clone, Copy)]
pub enum SymtabCode {
    Blob = 1,
}

#[derive(Debug, Clone, Copy)]
pub enum FastMathMap {
    UnsafeAlgebra = (1 << 0), // Legacy
    NoNaNs = (1 << 1),
    NoInfs = (1 << 2),
    NoSignedZeros = (1 << 3),
    AllowReciprocal = (1 << 4),
    AllowContract = (1 << 5),
    ApproxFunc = (1 << 6),
    AllowReassoc = (1 << 7),
}

#[derive(Debug, Clone, Copy)]
pub enum TruncInstOptionalFlag {
    NO_UNSIGNED_WRAP = 0,
    NO_SIGNED_WRAP = 1,
}

#[derive(Debug, Clone, Copy)]
pub enum OverFlowingBinOpOptionalFlag {
    NO_UNSIGNED_WRAP = 0,
    NO_SIGNED_WRAP = 1,
}

#[derive(Debug, Clone, Copy)]
pub enum MetadataCode {
    STRING_OLD = 1,              // MDSTRING:      [values]
    VALUE = 2,                   // VALUE:         [type num, value num]
    NODE = 3,                    // NODE:          [n x md num]
    NAME = 4,                    // STRING:        [values]
    DISTINCT_NODE = 5,           // DISTINCT_NODE: [n x md num]
    KIND = 6,                    // [n x [id, name]]
    LOCATION = 7,                // [distinct, line, col, scope, inlined-at?]
    OLD_NODE = 8,                // OLD_NODE:      [n x (type num, value num)]
    OLD_FN_NODE = 9,             // OLD_FN_NODE:   [n x (type num, value num)]
    NAMED_NODE = 10,             // NAMED_NODE:    [n x mdnodes]
    ATTACHMENT = 11,             // [m x [value, [n x [id, mdnode]]]
    GENERIC_DEBUG = 12,          // [distinct, tag, vers, header, n x md num]
    SUBRANGE = 13,               // [distinct, count, lo]
    ENUMERATOR = 14,             // [isUnsigned|distinct, value, name]
    BASIC_TYPE = 15,             // [distinct, tag, name, size, align, enc]
    FILE = 16,                   // [distinct, filename, directory, checksumkind, checksum]
    DERIVED_TYPE = 17,           // [distinct, ...]
    COMPOSITE_TYPE = 18,         // [distinct, ...]
    SUBROUTINE_TYPE = 19,        // [distinct, flags, types, cc]
    COMPILE_UNIT = 20,           // [distinct, ...]
    SUBPROGRAM = 21,             // [distinct, ...]
    LEXICAL_BLOCK = 22,          // [distinct, scope, file, line, column]
    LEXICAL_BLOCK_FILE = 23,     //[distinct, scope, file, discriminator]
    NAMESPACE = 24,              // [distinct, scope, file, name, line, exportSymbols]
    TEMPLATE_TYPE = 25,          // [distinct, scope, name, type, ...]
    TEMPLATE_VALUE = 26,         // [distinct, scope, name, type, value, ...]
    GLOBAL_VAR = 27,             // [distinct, ...]
    LOCAL_VAR = 28,              // [distinct, ...]
    EXPRESSION = 29,             // [distinct, n x element]
    OBJC_PROPERTY = 30,          // [distinct, name, file, line, ...]
    IMPORTED_ENTITY = 31,        // [distinct, tag, scope, entity, line, name]
    MODULE = 32,                 // [distinct, scope, name, ...]
    MACRO = 33,                  // [distinct, macinfo, line, name, value]
    MACRO_FILE = 34,             // [distinct, macinfo, line, file, ...]
    STRINGS = 35,                // [count, offset] blob([lengths][chars])
    GLOBAL_DECL_ATTACHMENT = 36, // [valueid, n x [id, mdnode]]
    GLOBAL_VAR_EXPR = 37,        // [distinct, var, expr]
    INDEX_OFFSET = 38,           // [offset]
    INDEX = 39,                  // [bitpos]
    LABEL = 40,                  // [distinct, scope, name, file, line]
    STRING_TYPE = 41,            // [distinct, name, size, align,...]
    // Codes 42 and 43 are reserved for support for Fortran array specific debug
    // info.
    COMMON_BLOCK = 44,     // [distinct, scope, name, variable,...]
    GENERIC_SUBRANGE = 45, // [distinct, count, lo, up, stride]
    ARG_LIST = 46,         // [n x [type num, value num]]
    ASSIGN_ID = 47,        // [distinct, ...]
}


pub mod vbr_widths {
    pub const ALIGNMENT: u32 = 3;
    pub const ARRAY_LENGTH: u32 = 8;
    pub const ATTR_INDEX: u32 = 6;
    pub const BLOCK_COUNT: u32 = 6;
    pub const BLOCK_INDEX: u32 = 8;
    pub const CCONV: u32 = 5;
    pub const INTEGER: u32 = 8;
    pub const INT_WIDTH: u32 = 8;
    pub const METADATA_INDEX: u32 = 6;
    pub const METADATA_KIND_INDEX: u32 = 6;
    pub const METADATA_STRING_COUNT: u32 = 6;
    pub const METADATA_STRING_OFF: u32 = 6;
    pub const STRTAB_LENGTH: u32 = 6;
    pub const STRTAB_OFFSET: u32 = 8;
    pub const TYPE_INDEX: u32 = 6;
    pub const VALUE_INDEX: u32 = 8;
}
