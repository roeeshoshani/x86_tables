pub type Mnemonic = &'static str;

pub const MNEMONIC_UNSUPPORTED: Mnemonic = "unsupported";

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum OpSize {
    S8,
    S16,
    S32,
    S64,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OpSizeInfo {
    pub with_operand_size_override: OpSize,
    pub mode_32: OpSize,
    pub mode_64: OpSize,
    pub mode_64_with_rex_w: OpSize,
}
impl OpSizeInfo {
    /// operand size is always 8 bits
    pub const SZ_ALWAYS_8: Self = Self {
        with_operand_size_override: OpSize::S8,
        mode_32: OpSize::S8,
        mode_64: OpSize::S8,
        mode_64_with_rex_w: OpSize::S8,
    };

    /// the default operand size for instructions that default to 32-bit operands.
    pub const SZ_16_32_64_DEF_32: Self = Self {
        with_operand_size_override: OpSize::S16,
        mode_32: OpSize::S32,
        mode_64: OpSize::S32,
        mode_64_with_rex_w: OpSize::S64,
    };

    /// the default operand size for instructions that default to 64-bit operands.
    pub const SZ_16_32_64_DEF_64: Self = Self {
        with_operand_size_override: OpSize::S16,
        mode_32: OpSize::S32,
        mode_64: OpSize::S64,
        mode_64_with_rex_w: OpSize::S64,
    };

    /// a common size info for immediate encodings that are either 16 or 32 bits.
    pub const SZ_IMM_ENCODING_16_32: Self = Self {
        with_operand_size_override: OpSize::S16,
        mode_32: OpSize::S32,
        mode_64: OpSize::S32,
        mode_64_with_rex_w: OpSize::S32,
    };
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ImmExtendKind {
    SignExtend,
    ZeroExtend,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ImmOpInfo {
    pub encoded_size: OpSizeInfo,
    pub extended_size: OpSizeInfo,
    pub extend_kind: ImmExtendKind,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SpecificImmOpInfo {
    pub value: u64,
    pub operand_size: OpSizeInfo,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MemOffsetOpInfo {
    pub mem_operand_size: OpSizeInfo,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RegEncoding {
    Modrm,
    Opcode,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RegOpInfo {
    pub encoding: RegEncoding,
    pub size: OpSizeInfo,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SpecificRegOpInfo {
    pub reg_64_bit_name: &'static str,
    pub size: OpSizeInfo,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ZextSpecificRegOpInfo {
    pub reg_64_bit_name: &'static str,
    pub size: OpSizeInfo,
    pub extended_size: OpSizeInfo,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RelOpInfo {
    pub size: OpSizeInfo,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum OpInfo {
    /// immediate operand
    Imm(ImmOpInfo),

    /// specific immediate which is enforced by the opcode
    SpecificImm(SpecificImmOpInfo),

    /// register operand
    Reg(RegOpInfo),

    /// rm operand
    Rm(OpSizeInfo),

    /// specific register which is enforced by the opcode
    SpecificReg(SpecificRegOpInfo),

    /// zero extended specific register which is enforced by the opcode
    ZextSpecificReg(ZextSpecificRegOpInfo),

    /// relative offset used for relative jumps
    Rel(OpSizeInfo),

    /// memory access by absolute address, for example `mov rcx, [0x1234]`
    MemOffset(MemOffsetOpInfo),

    /// an implicit operand which is not actually specified in the instruction, only its size it relevant.
    Implicit(OpSizeInfo),

    Cond,
}
impl OpInfo {
    pub const RM_8: Self = Self::Rm(OpSizeInfo::SZ_ALWAYS_8);
    pub const RM_16_32_64_DEF_32: Self = Self::Rm(OpSizeInfo::SZ_16_32_64_DEF_32);
    pub const RM_16_32_64_DEF_64: Self = Self::Rm(OpSizeInfo::SZ_16_32_64_DEF_64);
    pub const R_MODRM_8: Self = Self::Reg(RegOpInfo {
        encoding: RegEncoding::Modrm,
        size: OpSizeInfo::SZ_ALWAYS_8,
    });
    pub const R_MODRM_16_32_64_DEF_32: Self = Self::Reg(RegOpInfo {
        encoding: RegEncoding::Modrm,
        size: OpSizeInfo::SZ_16_32_64_DEF_32,
    });
    pub const R_OPCODE_8: Self = Self::Reg(RegOpInfo {
        encoding: RegEncoding::Opcode,
        size: OpSizeInfo::SZ_ALWAYS_8,
    });
    pub const R_OPCODE_16_32_64_DEF_32: Self = Self::Reg(RegOpInfo {
        encoding: RegEncoding::Opcode,
        size: OpSizeInfo::SZ_16_32_64_DEF_32,
    });
    pub const R_OPCODE_16_32_64_DEF_64: Self = Self::Reg(RegOpInfo {
        encoding: RegEncoding::Opcode,
        size: OpSizeInfo::SZ_16_32_64_DEF_64,
    });
    pub const AL: Self = Self::SpecificReg(SpecificRegOpInfo {
        size: OpSizeInfo::SZ_ALWAYS_8,
        reg_64_bit_name: "rax",
    });
    pub const AX_16_32_64_DEF_32: Self = Self::SpecificReg(SpecificRegOpInfo {
        size: OpSizeInfo::SZ_16_32_64_DEF_32,
        reg_64_bit_name: "rax",
    });
    pub const DX_16_32_64_DEF_32: Self = Self::SpecificReg(SpecificRegOpInfo {
        size: OpSizeInfo::SZ_16_32_64_DEF_32,
        reg_64_bit_name: "rdx",
    });
    pub const CL: Self = Self::SpecificReg(SpecificRegOpInfo {
        size: OpSizeInfo::SZ_ALWAYS_8,
        reg_64_bit_name: "rcx",
    });

    /// an 8-bit immediate which should not be sign/zero extended.
    pub const IMM_8_NO_EXT: Self = Self::Imm(ImmOpInfo {
        encoded_size: OpSizeInfo::SZ_ALWAYS_8,
        extended_size: OpSizeInfo::SZ_ALWAYS_8,
        // doesn't matter
        extend_kind: ImmExtendKind::SignExtend,
    });

    /// a 16/32 bit relative offset
    pub const REL_16_32: Self = Self::Rel(OpSizeInfo {
        // operand size override is not allowed with relative operands, so this is ignores anyway
        with_operand_size_override: OpSize::S16,
        mode_32: OpSize::S32,
        mode_64: OpSize::S64,
        mode_64_with_rex_w: OpSize::S64,
    });
}

pub type Ops = &'static [OpInfo];

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RegularInsnInfo {
    pub mnemonic: Mnemonic,
    pub ops: Ops,
}
impl RegularInsnInfo {
    pub const UNSUPPORTED: Self = Self {
        mnemonic: MNEMONIC_UNSUPPORTED,
        ops: &[],
    };
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ModrmRegOpcodeExtInsnInfo {
    pub by_reg_value: [RegularInsnInfo; 8],
}
impl ModrmRegOpcodeExtInsnInfo {
    pub fn new_with_same_operands(ops: Ops, mnemonics: [Mnemonic; 8]) -> Self {
        Self {
            by_reg_value: std::array::from_fn(|i| RegularInsnInfo {
                mnemonic: mnemonics[i],
                ops,
            }),
        }
    }
}

#[derive(Debug, Clone)]
pub enum InsnInfo {
    Regular(RegularInsnInfo),
    ModrmRegOpcodeExt(ModrmRegOpcodeExtInsnInfo),
}
